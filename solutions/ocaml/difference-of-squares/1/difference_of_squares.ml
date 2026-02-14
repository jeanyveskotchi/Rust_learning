let rec sum_to n =
  if n <= 0 then 0
  else n + sum_to (n - 1)

let square_of_sum n =
  let s = sum_to n in
  s * s

let rec sum_of_squares n =
  if n <= 0 then 0
  else (n * n) + sum_of_squares (n - 1)

let difference_of_squares n =
  square_of_sum n - sum_of_squares n
