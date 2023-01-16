
fn a() {
    let b = Xcm([
		WithdrawAsset(MultiAssets([MultiAsset {
			id: Concrete(MultiLocation {
				parents: 1,
				interior: Here,
			}),
			fun: Fungible(10000000000000000),
		}])),
		InitiateReserveWithdraw {
			assets: Wild(All),
			reserve: MultiLocation {
				parents: 1,
				interior: Here,
			},
			xcm: Xcm([
				BuyExecution {
					fees: MultiAsset {
						id: Concrete(MultiLocation {
							parents: 0,
							interior: Here,
						}),
						fun: Fungible(5000000000000000),
					},
					weight_limit: Unlimited,
				},
				DepositReserveAsset {
					assets: Wild(All),
					max_assets: 1,
					dest: MultiLocation {
						parents: 0,
						interior: X1(Parachain(2011)),
					},
					xcm: Xcm([
						BuyExecution {
							fees: MultiAsset {
								id: Concrete(MultiLocation {
									parents: 1,
									interior: Here,
								}),
								fun: Fungible(5000000000000000),
							},
							weight_limit: Unlimited,
						},
						DepositAsset {
							assets: Wild(All),
							max_assets: 1,
							beneficiary: MultiLocation {
								parents: 0,
								interior: X1(AccountId32 {
									network: Any,
									id: [
										1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
										1, 1, 1, 1, 1, 1,
									],
								}),
							},
						},
					]),
				},
			]),
		},
	]);
}