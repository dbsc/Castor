# Castor

```text
                     .d888888888888b.
                   .d8888888888888888b.
                  d888888888888888Y88888b.
                 d8888888888888888 " Y8888b
 .d8888888b.     88888888888888888    88888
d888888888888b..d888888888888888888  .8888P
Y888888888888888888888888888P  Y88888888P"
 "Y88888888P"   "Y88888888P"     │  │  │
                                 └─ .──┘


```

## What it does
Splits any file into secure chunks with 50% backup protection using Reed-Solomon math. If you lose some chunks, you can still fully restore the original file.


## How to install
1. Give permission to the installer script:
   ```bash
   chmod +x install.sh
   ```
2. Run the installer:
   ```bash
   ./install.sh
   ```

## How to use
Now you can use the `castor` command anywhere in your terminal!

* **To crash:**
  ```bash
  castor crash
  ```
* **To restore:**
  ```bash
  castor restore
  ```

