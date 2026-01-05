(defun fromhexstream (str)
  (with-input-from-string (is str)
    (loop for i from 0 below (length str) by 2 collect
      (parse-integer (subseq str i (min (+ i 2) (length str))) :radix 16))))

