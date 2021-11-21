# STM32F3Discovery Example

The structure for this example was generated using [knurling-rs/app-template](https://github.com/knurling-rs/app-template). It runs on an [STM32F3DISCOVERY board](https://www.st.com/en/evaluation-tools/stm32f3discovery.html).

## Setup

### Software

#### 1. `flip-link`:

```console
$ cargo install flip-link
```

#### 2. `probe-run`:

``` console
$ # make sure to install v0.2.0 or later
$ cargo install probe-run
```

### Hardware

The only required hardware is a GP2D12 sensor and an STM32F3DISCOVERY board.

Make the following connections:

| GP2D12         | STM32F3DISCOVERY |
|----------------|------------------|
| V<sub>CC</sub> | 5V               |
| GND            | GND              |
| V<sub>O</sub>  | PE7              |

## Running

```console
cargo run
```

You should see output like this:

```
distance: 31 cm
└─ stm32f3discovery_example::__cortex_m_rt_main @ src/main.rs:37
distance: 30 cm
└─ stm32f3discovery_example::__cortex_m_rt_main @ src/main.rs:37
distance: 29 cm
└─ stm32f3discovery_example::__cortex_m_rt_main @ src/main.rs:37
distance: 29 cm
└─ stm32f3discovery_example::__cortex_m_rt_main @ src/main.rs:37
distance: 26 cm
└─ stm32f3discovery_example::__cortex_m_rt_main @ src/main.rs:37
```
