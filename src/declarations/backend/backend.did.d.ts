import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface CaptchaRequirement {
  'is_alpha_numeric' : boolean,
  'char_count' : number,
}
export interface SolveCaptchaArgs { 'id' : bigint, 'ans' : string }
export interface _SERVICE {
  'generate_captcha' : ActorMethod<[CaptchaRequirement], [bigint, string]>,
  'solve_captcha' : ActorMethod<[SolveCaptchaArgs], boolean>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
