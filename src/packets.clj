#!/usr/bin/env clojure
(ns packets
  (:use [clojure.java.io]))
(load-file "packet_definitions.clj")

;;;; What this file does is given a bunch of packet definitions, create all
;;;; the .*.generated.rs files, which contain the definitions of packets
;;;; and the ClientboundPacket and ServerboundPacket enums.
;;;;
;;;; This way we only have to define packets in once, in packet_definitions.clj.
;;;; Is this the cleanest solution to the problem? No, but it works well, and
;;;; was easy to write, and it avoids adding additional dependencies for
;;;; building ozelot.

(defn long-str [& x] (clojure.string/join "\n" x))

(def clientbound-file "./.clientbound-packets.generated.rs")
(def serverbound-file "./.serverbound-packets.generated.rs")
(def clientbound-enum-file "./.clientbound-enum.generated.rs")
(def serverbound-enum-file "./.serverbound-enum.generated.rs")
(def warning (long-str
               "/* This file is automatically generated by packets.clj"
               "Do not manually edit this file, if you wish to make"
               "changes here, then edit and rerun packets.clj */\n\n"))
; Clear the files
(spit clientbound-file warning)
(spit serverbound-file warning)
(spit clientbound-enum-file warning)
(spit serverbound-enum-file warning)

(def clientstates ["Handshake" "Status" "Login" "Play"])

;; A list of the clientbound packets
(def clientbound-packets
  (->> packets
       :clientbound
       ; For each packet, we assoc in the :state (serverbound, clientbound)
       (map (fn [[key value]]
              (map (fn [packet]
                     (assoc packet :state key))
                   value)))
       (apply concat)))

;; A list of the serverbound packets
(def serverbound-packets
  (->> packets
       :serverbound
       ; For each packet, we assoc in the :state (serverbound, clientbound)
       (map (fn [[key value]]
              (map (fn [packet]
                     (assoc packet :state key))
                   value)))
       (apply concat)))


;; Given a name and some packets, provide the enum definition for said packets
(defn enum-definition [name packets]
  (format
    (long-str "/// Represents a single packet"
              "#[derive(Debug, PartialEq, Clone)]"
              "pub enum %s {"
              (apply str
                     (for [{name :name} packets]
                       (format "    %s(%s),\n" name name)
                       ))
              "}"
              ""
              "")
    name))

(spit clientbound-enum-file (enum-definition "ClientboundPacket" clientbound-packets) :append true)
(spit serverbound-enum-file (enum-definition "ServerboundPacket" serverbound-packets) :append true)

;; Creates the whole match id block for use in the enum's parse function
;; Should only be called by enum-fn-deserialize
(defn enum-fn-deserialize-id [state packets]
  (format
    (long-str "        &ClientState::%s => {"
              (if (empty? packets)
                "            Err(\"No packet available in this state\".into())\n"
                (long-str "            match packet_id {"
                          (apply str
                                 (for [{name :name id :id} packets]
                                   (format "            %s => Ok(%s::deserialize(r)?),\n" id name)))
                          "            _ => bail!(\"No packet with id {} in state {}\", packet_id, state),"
                          "            }"))
              "        },"
              "")
    state))

;; Creates the whole match state block for use in the enum's parse function
;; Should only be called by enum-fn-deserialize
(defn enum-fn-deserialize-state [packets]
  (long-str "        match state {"
            (apply str
                   (for [state clientstates]
                     (enum-fn-deserialize-id state (filter (fn [p] (= (p :state) state)) packets))))
            "        }"))

;; Create the parse function for the Packet trait for the given packets
(defn enum-fn-deserialize [packets]
  (long-str "    pub fn deserialize<R: Read>(r: &mut R, state: &ClientState) -> Result<Self> {"
            "        let packet_id = read_varint(r)?;"
            (enum-fn-deserialize-state packets)
            "    }"))

;; Create the get_packet_name function for the Packet trait for the given packets
(defn enum-fn-get-packet-name [packets packet-type]
  (long-str "    pub fn get_packet_name(&self) -> &str {"
            "        match self {"
            (apply str
                   (for [{name :name} packets]
                     (format "        &%s::%s(..) => \"%s\",\n"
                             packet-type name name)))
            "        }"
            "    }"))

;; Create the get_state function for the Packet trait for the given packets
(defn enum-fn-get-state [packets packet-type]
  (long-str "    pub fn get_clientstate(&self) -> ClientState {"
            "        match self {"
            (apply str
                   (for [{name :name state :state} packets]
                     (format "        &%s::%s(..) => ClientState::%s,\n"
                             packet-type name state)))
            "        }"
            "    }"))

;; Create the get_id function for the Packet trait for the given packets
(defn enum-fn-get-id [packets packet-type]
  (long-str "    pub fn get_id(&self) -> i32 {"
            "        match self {"
            (apply str
                   (for  [{name :name id :id} packets]
                     (format "        &%s::%s(..) => %s,\n"
                             packet-type name id)))
            "        }"
            "    }"))

;; Create the to_u8 function for the Packet trait for the given packets
(defn enum-fn-to-u8 [packets packet-type]
  (long-str "    pub fn to_u8(&self) -> Result<Vec<u8>> {"
            "        match self {"
            (apply str
                   (for [{name :name} packets]
                     (format "        &%s::%s(ref x) => x.to_u8(),\n"
                             packet-type name)))
            "        }"
            "    }"))

(defn enum-impl-packet [packet-type packets]
  (format
    (long-str "impl Packet for %s {"
              (enum-fn-deserialize packets)
              (enum-fn-get-packet-name packets packet-type)
              (enum-fn-get-state packets packet-type)
              (enum-fn-get-id packets packet-type)
              (enum-fn-to-u8 packets packet-type)
              "}"
              "impl fmt::Display for %s {"
              "    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"
              "        write!(f, \"%s of type {}\", self.get_packet_name())"
              "    }"
              "}")
    packet-type
    packet-type
    packet-type))

(spit clientbound-enum-file (enum-impl-packet "ClientboundPacket" clientbound-packets) :append true)
(spit serverbound-enum-file (enum-impl-packet "ServerboundPacket" serverbound-packets) :append true)

;; Given the fields of a given packet, return a string containing the fields
;; in the rust definition format, i.e. %name: %type, and so on
(defn fields-type-str [fields]
  (apply str
         (map
           (fn [{x :name y :type}]
             (format "    %s: %s,\n" x y))
           fields)))

;; Given the fields of a given packet, return a string containing the fields
;; in the %name: %name format, e.g. for use in an fn new
(defn fields-name-str [fields]
  (apply str
         (map
           (fn [{name :name}]
             (format "            %s: %s,\n" name name))
           fields)))

;; Given the fields of a given packet, return a string containing the fields
;; in the rust packet definition format, i.e. fn something(%s: %s, %s: %s, ... etc)
;; (i.e it returns everything within the parenthese)
(defn fields-fn-definition-str [fields]
  (apply (fn [& x] (clojure.string/join ", " x))
         (map
           (fn [{name :name type :type}]
             (format "%s: %s" name type))
           fields)))

;; Given some fields, create the write calls to each of the respective data types
;; for each of the fields.
(defn write-fields-str [fields]
  (apply str
         (map
           (fn [{name :name type :type read :read}]
             (format "        write_%s(&self.%s, &mut ret)?;\n"
                     (if (nil? read)
                       type
                       read)
                     name))
           fields)))

;; Create the const PACKET_ID fields
(defn const-packet-id [{id :id}]
  (format "    const PACKET_ID: i32 = %s;" id))

;; Given the fields of a packet, return a string containing functions that
;; read all those fields, i.e. for use inside a 'new' function for the packet
(defn read-fields-str [fields]
  (apply str
         (map
           (fn [{name :name type :type read :read}]
             (format "            %s: read_%s(r)?,\n"
                     name
                     ; If :read is specified use that function to read from
                     ; else use the read_:type function
                     (if (nil? read)
                       type
                       read)))
           fields)))

;; Create the deserialize function for a packet
(defn fn-deserialize [packet packet-type]
  (let [{name :name fields :fields automatic-serialize :automatic-serialize} packet]
    (format
      (long-str "    pub fn deserialize<R: Read>(r: &mut R) -> Result<%sPacket> {"
                "        Ok(%sPacket::%s(%s {"
                (read-fields-str fields)
                "        }))"
                "    }")
      packet-type packet-type name name)))

;; Create the to_u8 function for a packet
(defn fn-to-u8 [{name :name fields :fields automatic-serialize :automatic-serialize}]
  (if (nil? automatic-serialize)
    (long-str "    pub fn to_u8(&self) -> Result<Vec<u8>> {"
              "        let mut ret = Vec::new();"
              "        write_varint(&Self::PACKET_ID, &mut ret)?;"
              (write-fields-str fields)
              "        Ok(ret)"
              "    }")
    ""))

;; Crates the fn new for a packet
(defn fn-new [{fields :fields name :name automatic-serialize :automatic-serialize} packet-type]
  (format (long-str "    pub fn %s(%s) -> %sPacket {"
                    "        %sPacket::%s(%s {"
                    "%s        })"
                    "    }"
                    )
          ; If automatic-serialize is false, we call the function new_raw instead of new
          (if (nil? automatic-serialize) "new" "new_raw")
          (fields-fn-definition-str fields)
          packet-type
          packet-type
          name
          name
          (fields-name-str fields)))

;; Given a packet create the getter functions for its publically accessible fields
(defn packet-getter-fns [{fields :fields}]
  (apply str
         (for [{name :name type :type getter-docs :getter} fields]
           (format
             (long-str "    /// %s"
                       "    pub fn get_%s(&self) -> &%s {"
                       "        &self.%s"
                       "    }")
             (or getter-docs (format "get the %s field (UNDOCUMENTED)" name)) name type name))))

;; Create the impl for a given packet
(defn packet-impl [packet packet-type]
  (let [{name :name automatic-serialize :automatic-serialize} packet]
    (long-str (format "impl %s {" name)
              (const-packet-id packet)
              (when (nil? automatic-serialize) (fn-deserialize packet packet-type))
              (fn-to-u8 packet)
              (fn-new packet packet-type)
              (packet-getter-fns packet)
              "}"
              ""
              "")))

;; Provides the struct definitions for all the given packets
(defn packet-definitions [packets packet-type]
  (apply str
         (for [packet packets]
           (let [{name :name automatic-serialize :automatic-serialize fields :fields} packet
                 fields-str (fields-type-str fields)
                 struct-def (format "#[derive(Debug, PartialEq, Clone)]\npub struct %s {\n%s}\n\n" name fields-str)
                 impl-def (packet-impl packet packet-type)]
             (str struct-def impl-def)
             ))))

(spit clientbound-file (packet-definitions clientbound-packets "Clientbound") :append true)
(spit serverbound-file (packet-definitions serverbound-packets "Serverbound") :append true)

