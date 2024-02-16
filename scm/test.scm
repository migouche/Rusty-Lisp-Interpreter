(defun test-scope3 ()
  (let ((myvar (if (ignore-errors myvar)
                   myvar
                   100)))
    (defun set-var3 ()
      (setf myvar (+ myvar 100)))
    (set-var3)))