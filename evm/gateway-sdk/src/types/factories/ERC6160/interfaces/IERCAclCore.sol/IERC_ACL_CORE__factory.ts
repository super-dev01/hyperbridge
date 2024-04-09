/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */

import { Contract, Interface, type ContractRunner } from "ethers";
import type {
  IERC_ACL_CORE,
  IERC_ACL_COREInterface,
} from "../../../../ERC6160/interfaces/IERCAclCore.sol/IERC_ACL_CORE";

const _abi = [
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "role",
        type: "bytes32",
      },
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
    ],
    name: "grantRole",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "role",
        type: "bytes32",
      },
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
    ],
    name: "hasRole",
    outputs: [
      {
        internalType: "bool",
        name: "",
        type: "bool",
      },
    ],
    stateMutability: "view",
    type: "function",
  },
  {
    inputs: [
      {
        internalType: "bytes32",
        name: "role",
        type: "bytes32",
      },
      {
        internalType: "address",
        name: "account",
        type: "address",
      },
    ],
    name: "revokeRole",
    outputs: [],
    stateMutability: "nonpayable",
    type: "function",
  },
] as const;

export class IERC_ACL_CORE__factory {
  static readonly abi = _abi;
  static createInterface(): IERC_ACL_COREInterface {
    return new Interface(_abi) as IERC_ACL_COREInterface;
  }
  static connect(
    address: string,
    runner?: ContractRunner | null
  ): IERC_ACL_CORE {
    return new Contract(address, _abi, runner) as unknown as IERC_ACL_CORE;
  }
}
