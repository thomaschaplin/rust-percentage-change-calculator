<img src="assets/logo.png" alt="logo" width="240"/>

# rust-percentage-change-calculator

Percentage change calculator written in [rust](https://www.rust-lang.org/)

## What is percentage change?

A percentage change is a way to express a change in a variable. It represents the relative change between the old value and the new one.

For example, if a house is worth $100,000 today and the year after its value goes up to $110,000, the percentage change of its value can be expressed as

![](assets/house_example.svg)

It can then be said that the worth of the house went up by 10%.

## How do you calculate the percentage change?

![](/assets/percent_change_formula.svg)

In simple terms, there are three calculations that we carry out, here's an example:

The first number minus the second number
The sum of the previous calculation divided by the first number
The sum of the previous calculation multipled by 100
The answer is the percentage changed

```
50 - 10 = 40
40 / 50 = 0.8
0.8 * 100 = -8%
```

OR

```
100 - 200 = 100
100 / 200 = 0.5
0.5 * 100 = +50%
```


## Docker Usage

- `docker pull thomaschaplin/rust-percentage-change-calculator`
- `docker run --rm thomaschaplin/rust-percentage-change-calculator <NUMBER> <NUMBER>`

## Development Setup

Make sure you have [rust](https://www.rust-lang.org/) installed on your machine by following the [getting started guide](https://www.rust-lang.org/learn/get-started)

### Instructions

* Clone this repository `git clone git@github.com:thomaschaplin/rust-percentage-change-calculator.git`
* Change directory `cd rust-percentage-change-calculator`
* Build the application `cargo build`
* Run the application `cargo run <NUMBER> <NUMBER>`

### Final Build

* Build the application in release mode `cargo build --release`
* Execute the `rust-percentage-change-calculator` binary file found in `target/release/rust-percentage-change-calculator`

#### Example Usage:

`./rust-percentage-change-calculator 100 200`

```
+100%
```

`./rust-percentage-change-calculator 200 100`

```
-50%
```

`./rust-percentage-change-calculator 100 100`

```
0%
```

### Local Docker Setup

Build
```
docker build --rm -f Dockerfile -t thomaschaplin/rust-percentage-change-calculator .
```

Run
```
docker run --rm thomaschaplin/rust-percentage-change-calculator <NUMBER> <NUMBER>
```

---

[Percentage](https://www.clipartkey.com/view/iRmhoxw_percentage-png-photos-percentage-flat-icon/) graphic by <a href="https://www.clipartkey.com/upic/4217/">Mario Rui</a> from ClipArtKey.
