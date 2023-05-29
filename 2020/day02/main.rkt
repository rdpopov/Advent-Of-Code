#lang racket
(require racket/local)

(define inp-file "./input")
(define (task-input fname) (file->lines fname))

(define (parse-line l)
  (remove "" (string-split l #rx":|-| " ))
  )



(define (verify-password ln)
  (define pas (parse-line ln))
  #| (define counted (- (apply string-length (cdddr pas)) |#
  #|                      (string-length (remove (caddr pas) (cdddr pas))) |#
  #|                      )) |#
  (print (apply remove (cadr) (cdddr pas)))
  #| (println (apply string-length (cdddr pas))) |#
  #| (println (apply string-length (remove (caddr pas) (cadddr pas)))) |#
  (cond
    [(< (string->number (car pas)) 0 (string->number (cadr pas))) 1]
    [true 0]
    )
  )

(define (part1 inp) (foldr + 0 (map verify-password inp)))
#| (print (part1 (task-input inp-file))) |#
(print (verify-password "2-9 c: ccccccccc" ))
(print (verify-password "1-3 a: abcde" ))

