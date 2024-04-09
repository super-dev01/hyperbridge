/* Autogenerated file. Do not edit manually. */
/* tslint:disable */
/* eslint-disable */
import type {
  BaseContract,
  BigNumberish,
  BytesLike,
  FunctionFragment,
  Result,
  Interface,
  ContractRunner,
  ContractMethod,
  Listener,
} from "ethers";
import type {
  TypedContractEvent,
  TypedDeferredTopicFilter,
  TypedEventLog,
  TypedListener,
  TypedContractMethod,
} from "../../common";

export type PostRequestStruct = {
  source: BytesLike;
  dest: BytesLike;
  nonce: BigNumberish;
  from: BytesLike;
  to: BytesLike;
  timeoutTimestamp: BigNumberish;
  body: BytesLike;
};

export type PostRequestStructOutput = [
  source: string,
  dest: string,
  nonce: bigint,
  from: string,
  to: string,
  timeoutTimestamp: bigint,
  body: string
] & {
  source: string;
  dest: string;
  nonce: bigint;
  from: string;
  to: string;
  timeoutTimestamp: bigint;
  body: string;
};

export type GetRequestStruct = {
  source: BytesLike;
  dest: BytesLike;
  nonce: BigNumberish;
  from: BytesLike;
  timeoutTimestamp: BigNumberish;
  keys: BytesLike[];
  height: BigNumberish;
};

export type GetRequestStructOutput = [
  source: string,
  dest: string,
  nonce: bigint,
  from: string,
  timeoutTimestamp: bigint,
  keys: string[],
  height: bigint
] & {
  source: string;
  dest: string;
  nonce: bigint;
  from: string;
  timeoutTimestamp: bigint;
  keys: string[];
  height: bigint;
};

export type StorageValueStruct = { key: BytesLike; value: BytesLike };

export type StorageValueStructOutput = [key: string, value: string] & {
  key: string;
  value: string;
};

export type GetResponseStruct = {
  request: GetRequestStruct;
  values: StorageValueStruct[];
};

export type GetResponseStructOutput = [
  request: GetRequestStructOutput,
  values: StorageValueStructOutput[]
] & { request: GetRequestStructOutput; values: StorageValueStructOutput[] };

export type PostResponseStruct = {
  request: PostRequestStruct;
  response: BytesLike;
  timeoutTimestamp: BigNumberish;
};

export type PostResponseStructOutput = [
  request: PostRequestStructOutput,
  response: string,
  timeoutTimestamp: bigint
] & {
  request: PostRequestStructOutput;
  response: string;
  timeoutTimestamp: bigint;
};

export interface BaseIsmpModuleInterface extends Interface {
  getFunction(
    nameOrSignature:
      | "onAccept"
      | "onGetResponse"
      | "onGetTimeout"
      | "onPostRequestTimeout"
      | "onPostResponse"
      | "onPostResponseTimeout"
  ): FunctionFragment;

  encodeFunctionData(
    functionFragment: "onAccept",
    values: [PostRequestStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "onGetResponse",
    values: [GetResponseStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "onGetTimeout",
    values: [GetRequestStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "onPostRequestTimeout",
    values: [PostRequestStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "onPostResponse",
    values: [PostResponseStruct]
  ): string;
  encodeFunctionData(
    functionFragment: "onPostResponseTimeout",
    values: [PostResponseStruct]
  ): string;

  decodeFunctionResult(functionFragment: "onAccept", data: BytesLike): Result;
  decodeFunctionResult(
    functionFragment: "onGetResponse",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "onGetTimeout",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "onPostRequestTimeout",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "onPostResponse",
    data: BytesLike
  ): Result;
  decodeFunctionResult(
    functionFragment: "onPostResponseTimeout",
    data: BytesLike
  ): Result;
}

export interface BaseIsmpModule extends BaseContract {
  connect(runner?: ContractRunner | null): BaseIsmpModule;
  waitForDeployment(): Promise<this>;

  interface: BaseIsmpModuleInterface;

  queryFilter<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;
  queryFilter<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    fromBlockOrBlockhash?: string | number | undefined,
    toBlock?: string | number | undefined
  ): Promise<Array<TypedEventLog<TCEvent>>>;

  on<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  on<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  once<TCEvent extends TypedContractEvent>(
    event: TCEvent,
    listener: TypedListener<TCEvent>
  ): Promise<this>;
  once<TCEvent extends TypedContractEvent>(
    filter: TypedDeferredTopicFilter<TCEvent>,
    listener: TypedListener<TCEvent>
  ): Promise<this>;

  listeners<TCEvent extends TypedContractEvent>(
    event: TCEvent
  ): Promise<Array<TypedListener<TCEvent>>>;
  listeners(eventName?: string): Promise<Array<Listener>>;
  removeAllListeners<TCEvent extends TypedContractEvent>(
    event?: TCEvent
  ): Promise<this>;

  onAccept: TypedContractMethod<
    [arg0: PostRequestStruct],
    [void],
    "nonpayable"
  >;

  onGetResponse: TypedContractMethod<
    [arg0: GetResponseStruct],
    [void],
    "nonpayable"
  >;

  onGetTimeout: TypedContractMethod<
    [arg0: GetRequestStruct],
    [void],
    "nonpayable"
  >;

  onPostRequestTimeout: TypedContractMethod<
    [arg0: PostRequestStruct],
    [void],
    "nonpayable"
  >;

  onPostResponse: TypedContractMethod<
    [arg0: PostResponseStruct],
    [void],
    "nonpayable"
  >;

  onPostResponseTimeout: TypedContractMethod<
    [arg0: PostResponseStruct],
    [void],
    "nonpayable"
  >;

  getFunction<T extends ContractMethod = ContractMethod>(
    key: string | FunctionFragment
  ): T;

  getFunction(
    nameOrSignature: "onAccept"
  ): TypedContractMethod<[arg0: PostRequestStruct], [void], "nonpayable">;
  getFunction(
    nameOrSignature: "onGetResponse"
  ): TypedContractMethod<[arg0: GetResponseStruct], [void], "nonpayable">;
  getFunction(
    nameOrSignature: "onGetTimeout"
  ): TypedContractMethod<[arg0: GetRequestStruct], [void], "nonpayable">;
  getFunction(
    nameOrSignature: "onPostRequestTimeout"
  ): TypedContractMethod<[arg0: PostRequestStruct], [void], "nonpayable">;
  getFunction(
    nameOrSignature: "onPostResponse"
  ): TypedContractMethod<[arg0: PostResponseStruct], [void], "nonpayable">;
  getFunction(
    nameOrSignature: "onPostResponseTimeout"
  ): TypedContractMethod<[arg0: PostResponseStruct], [void], "nonpayable">;

  filters: {};
}
