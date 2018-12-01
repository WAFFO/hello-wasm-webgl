#!/bin/sh

set -ex
cd "$(dirname $0)/www"

npm install
npm run serve

cd "$(dirname $0)"