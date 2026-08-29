#!/bin/bash

echo "=== Building Castor Utility in Release Mode ==="
cargo build --release

if [ $? -eq 0 ]; then
  echo "=== Compilation Success! Copying to system... ==="

  sudo cp target/release/Castor /usr/local/bin/castor

  echo "=== DONE! Now you can use 'castor crash' or 'castor restore' anywhere! ==="
else
  echo "Error: Compilation failed!"
  exit 1
fi
