(asdf:defsystem "inventory"
  :description "Example protobuf messages for testing cl-protobufs"
  :defsystem-depends-on (:cl-protobufs.asdf)
  :depends-on (:cl-protobufs)
  :components
  ((:protobuf-source-file "inventory"
    :proto-pathname "inventory.proto")))
