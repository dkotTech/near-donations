package services

import (
	"context"
	"encoding/json"
	"errors"
	"math/big"
	"strings"

	"github.com/ethereum/go-ethereum/rpc"
	api "github.com/textileio/near-api-go"
	"github.com/textileio/near-api-go/keys"
	"github.com/textileio/near-api-go/types"
)

var (
	OneNear, _   = big.NewInt(0).SetString("1000000000000000000000000", 10)
	OneNearFloat = big.NewFloat(0).SetInt(OneNear)
)

func ConnectToNear(ctx context.Context, rpcUrl, network string) (*api.Client, error) {
	rpcClient, err := rpc.DialContext(ctx, rpcUrl)
	if err != nil {
		return nil, err
	}

	keyPair, err := keys.NewKeyPairFromString(
		"ed25519:...",
	)

	config := &types.Config{
		RPCClient: rpcClient,
		Signer:    keyPair,
		NetworkID: network,
	}

	client, err := api.NewClient(config)
	if err != nil {
		return nil, err
	}

	return client, nil
}

func GetDonationsSequence(ctx context.Context, client *api.Client, contractID string) (*big.Int, error) {
	res, err := client.CallFunction(ctx, contractID, "donations_sequence",
		api.CallFunctionWithFinality("final"))
	if err != nil {
		return nil, err
	}

	seq, ok := big.NewInt(0).SetString(strings.ReplaceAll(string(res.Result), `"`, ""), 10)
	if !ok {
		return nil, errors.New("can not parse donations_sequence")
	}

	return seq, nil
}

func GetDonation(ctx context.Context, client *api.Client, contractID, donationID string) (*Donation, error) {
	var ok bool
	res, err := client.CallFunction(ctx, contractID, "get_donation",
		api.CallFunctionWithFinality("final"),
		api.CallFunctionWithArgs(map[string]string{"donation_id": donationID}))
	if err != nil {
		return nil, err
	}

	var donation Donation
	err = json.Unmarshal(res.Result, &donation)
	if err != nil {
		return nil, err
	}

	donation.BigSum, ok = big.NewInt(0).SetString(donation.Sum, 10)
	if !ok {
		return nil, errors.New("can not parse donation Sum")
	}

	return &donation, nil
}

type Donation struct {
	AccountID string   `json:"account_id"`
	Sum       string   `json:"sum"`
	BigSum    *big.Int `json:"-"`
	Msg       string   `json:"msg"`
}

func (d *Donation) ToCSV() []string {
	return []string{d.AccountID, YoctoToNEAR(d.BigSum), d.Msg}
}

func YoctoToNEAR(n *big.Int) string {
	sum := big.NewFloat(0).SetInt(big.NewInt(0).Set(n))
	return sum.Quo(sum, OneNearFloat).String()
}
