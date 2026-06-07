# Leap Years

As a user, I want to know if a year is a leap year, so that I can plan for an extra day on February 29th during those years.

## How it works

Acceptance Criteria:

- All years divisible by 400 ARE leap years (so, for example, 2000 was indeed a leap year)
- All years divisible by 100 but not by 400 are NOT leap years (so, for example, 1700, 1800, and 1900 were NOT leap years, NOR will 2100 be a leap year)
- All years divisible by 4 but not by 100 ARE leap years (e.g., 2008, 2012, 2016)
- All years not divisible by 4 are NOT leap years (e.g. 2017, 2018, 2019)

## Run

```bash
cargo run
```

Then enter a year when prompted.

## Example

```text
Please enter a year : 
2000
2000 is a leap year !

Please enter a year : 
1700
1700 is not a leap year ...

Please enter a year : 
1900
1900 is not a leap year ...

Please enter a year : 
2104
2104 is a leap year !

Please enter a year : 
2013
2013 is not a leap year ...

Please enter a year : 
2028
2028 is a leap year !
```