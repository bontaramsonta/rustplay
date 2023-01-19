def subarraysDivByK(A, K):
  # Edge case: if K is zero, return zero
  if K == 0:
    return 0
  # Edge case: if K is negative, make it positive before doing the calculation
  if K < 0:
    K = -K
  # Edge case: if the array is empty, return zero
  if len(A) == 0:
    return 0

  # Initialize a hashmap to keep track of the frequency of each remainder
  remainder_count = {}
  remainder_count[0] = 1

  # Initialize a variable to keep track of the prefix sum
  prefix_sum = 0
  count = 0

  # Iterate through the array
  for num in A:
    prefix_sum = (prefix_sum + num) % K
    if prefix_sum < 0:
      prefix_sum += K

    if prefix_sum in remainder_count:
      count += remainder_count[prefix_sum]
      remainder_count[prefix_sum] += 1
    else:
      remainder_count[prefix_sum] = 1

  return count
print(subarraysDivByK([4,5,0,-2,-3,1],5))
