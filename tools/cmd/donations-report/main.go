package main

import (
	"context"
	"encoding/csv"
	"flag"
	"log"
	"math/big"
	"os"
	"time"

	"github.com/schollz/progressbar/v3"
	"tools/services"
)

var (
	ContractID = flag.String("contract", "qawsew.testnet", "contract id for donation report")
	Net        = flag.String("net", "testnet", "near network")
	RPCUrl     = flag.String("rpc", "https://rpc.testnet.near.org", "near rpc url")
)

func main() {
	ctx, can := context.WithCancel(context.Background())
	defer can()

	client, err := services.ConnectToNear(ctx, *RPCUrl, *Net)
	if err != nil {
		log.Fatal(err)
	}

	seq, err := services.GetDonationsSequence(ctx, client, *ContractID)
	if err != nil {
		log.Fatal(err)
	}

	bar := progressbar.Default(seq.Int64() - 1)

	fileName := time.Now().Format("01-02-2006 15:04:05") + ".csv"

	file, err := os.Create(fileName)
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	w := csv.NewWriter(file)
	defer w.Flush()

	w.Write([]string{"From Account ID", "Donation Sum NEAR", "Message"})

	total := big.NewInt(0)

	for i := big.NewInt(0); i.Cmp(seq) < 0; i.Add(i, big.NewInt(1)) {
		donation, err := services.GetDonation(ctx, client, *ContractID, i.String())
		if err != nil {
			log.Fatal(err)
		}

		total = total.Add(total, donation.BigSum)

		w.Write(donation.ToCSV())
		bar.Add(1)
	}

	w.Write([]string{"total", services.YoctoToNEAR(total)})
}
