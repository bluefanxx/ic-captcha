export const idlFactory = ({ IDL }) => {
  const CaptchaRequirement = IDL.Record({
    'is_alpha_numeric' : IDL.Bool,
    'char_count' : IDL.Nat8,
  });
  const SolveCaptchaArgs = IDL.Record({ 'id' : IDL.Nat, 'ans' : IDL.Text });
  return IDL.Service({
    'generate_captcha' : IDL.Func(
        [CaptchaRequirement],
        [IDL.Nat, IDL.Text],
        [],
      ),
    'solve_captcha' : IDL.Func([SolveCaptchaArgs], [IDL.Bool], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
