import { createStore } from "solid-js/store";
import { CaptchaRequirement } from "./declarations/backend/backend.did";

const [captchaConfig, setCaptchaConfig] = createStore<CaptchaRequirement>({
  is_alpha_numeric: true,
  char_count: 5,
});

export { captchaConfig, setCaptchaConfig };
