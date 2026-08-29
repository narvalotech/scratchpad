(ql:quickload :cl-protobufs)
;; (ql:quickload :clunit2)
;; (ql:quickload '(:clunit2 :trivial-benchmark :cl-base64 :local-time :babel))
;; (asdf:test-system :cl-protobufs)

;; (search "/.local/bin"
;;         (sb-ext:posix-getenv "PATH"))
;;  ; => 9 (4 bits, #x9, #o11, #b1001)

(ql:quickload :inventory)

(defpackage #:my-inventory-test
  (:use #:common-lisp)
  (:local-nicknames (#:inv #:cl-protobufs.inventory)))

(in-package #:my-inventory-test)

(defvar *addr* (inv:make-address :street "123 Main St" :city "Springfield" :zip "00000"))

(defvar *item1* (inv:make-item :name "Laptop" :category :electronics
                               :price 999.99d0
                               :warehouse-ids '(1 4 7)))

(defvar *item2* (inv:make-item :name "Bananas" :category :grocery
                               :price 0.59d0
                               :warehouse-ids '(2)))

(defvar *order* (inv:make-order :order-id "ORD-001"
                                :shipping-address *addr*
                                :items (list *item1* *item2*)
                                :tracking-codes '(1001 1002)))

(defvar *bytes* (cl-protobufs:serialize-to-bytes *order*))
(defvar *decoded* (cl-protobufs:deserialize-from-bytes 'inv:order *bytes*))

(cl-protobufs:proto-equal *order* *decoded*)  

(defun bytes-to-hex-string (bytes)
  (with-output-to-string (s)
    (loop for byte across bytes
          for first = t then nil
          do (unless first (write-char #\Space s))
             (format s "~(~2,'0x~)" byte))))

(bytes-to-hex-string *bytes*)
                                        ; => "0a 07 4f 52 44 2d 30 30 31 12 21 0a 0b 31 32 33 20 4d 61 69 6e 20 53 74 12 0b 53 70 72 69 6e 67 66 69 65 6c 64 1a 05 30 30 30 30 30 1a 18 0a 06 4c 61 70 74 6f 70 10 01 19 52 b8 1e 85 eb 3f 8f 40 22 03 01 04 07 1a 17 0a 07 42 61 6e 61 6e 61 73 10 02 19 e1 7a 14 ae 47 e1 e2 3f 22 01 02 22 04 e9 07 ea 07"
