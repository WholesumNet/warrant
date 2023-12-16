
# Wholesum network `verification` tool

## Overview

Wholesum network is a p2p verifiable computing network. It builds on top of [Risc0](https://risczero.com/), [Swarm](https://ethswarm.org), [FairOS-dfs](https://github.com/fairDataSociety/fairOS-dfs), and [Libp2p](https://libp2p.io) to facilitate verifiable computing at scale.  

## Intro

Warrant is verification application that given a Risc0 `receipt` and an `Image Id`, checks if the execution yielding the receipt has been honest.

## USAGE

<pre>

Usage: warrant --image-id <IMAGE_ID> --receipt-file <RECEIPT_FILE>

Options:
  -i, --image-id <IMAGE_ID>          
  -r, --receipt-file <RECEIPT_FILE>  
  -h, --help                         Print help
  -V, --version                      Print version


</pre>
