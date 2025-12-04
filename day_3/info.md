The batteries are arranged into **banks**; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on **exactly two** batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like `12345` and you turn on batteries `2` and `4`, the bank would produce **24** jolts. (You cannot rearrange batteries.)

You'll need to find the largest possible joltage each bank can produce. In the above example:

- In `**98**7654321111111`, you can make the largest joltage possible, **98**, by turning on the first two batteries.
- In `8**11**111111111119`, you can make the largest joltage possible by turning on the batteries labeled **8** and **9**, producing **89** jolts.
- In `2342342342342**78**`, you can make **78** by turning on the last two batteries (marked **7** and **8**).
- In `818181911112**11**1`, the largest joltage you can produce is **92**.

The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is  
**98 + 89 + 78 + 92 = 357**.
