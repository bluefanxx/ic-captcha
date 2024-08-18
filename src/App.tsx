import { createStore } from "solid-js/store";
import { Show } from "solid-js";
import { AiFillGithub } from "solid-icons/ai";
import { CaptchaRequirement } from "./declarations/backend/backend.did";
import { createSignal } from "solid-js";
import { backend } from "./declarations/backend";
import "./App.css";

const [captchaStore, setCaptchaStore] = createStore<CaptchaRequirement>({
  is_alpha_numeric: true,
  char_count: 5,
});

const App = () => {
  const [captchaImage, setCaptchaImage] = createSignal<string | null>(null);
  const [captchaId, setCaptchaId] = createSignal<bigint | null>(null);
  const [answer, setAnswer] = createSignal("");

  const generateCaptcha = async () => {
    console.log("function called");
    const result = await backend.generate_captcha(captchaStore);
    console.log(result);
    setCaptchaId(result[0]);
    setCaptchaImage(result[1]);
  };

  const handleSubmit = async () => {
    if (captchaId()) {
      const result = await backend.solve_captcha({
        id: captchaId()!,
        ans: answer(),
      });
      console.log(result);
      alert(result ? "Correct!" : "Incorrect. Try again.");
    }
  };

  return (
    <div class="captcha-demo">
      <header class="captcha-demo-header">
        <h1 class="captcha-demo-title">Ic Captcha Demo</h1>
        <div class="captcha-demo-icons">
          <a
            href="https://github.com/bluefanxx/ic-captcha"
            target="_blank"
            rel="noopener noreferrer"
            class="captcha-demo-icon"
          >
            <AiFillGithub />
          </a>
        </div>
      </header>
      <main class="captcha-demo-main">
        <p class="captcha-demo-description">
          This is a demo for an onchain captcha service built on ICP.
        </p>
        <div class="captcha-demo-settings">
          <div class="captcha-demo-setting">
            <label for="char-count">Character Count:</label>
            <input
              type="range"
              id="char-count"
              min="5"
              max="10"
              value={captchaStore.char_count}
              onInput={(e) =>
                setCaptchaStore("char_count", parseInt(e.currentTarget.value))
              }
            />
            <span>{captchaStore.char_count}</span>
          </div>
          <div class="captcha-demo-setting">
            <label for="is-alpha-numeric">Alphanumeric:</label>
            <input
              type="checkbox"
              id="is-alpha-numeric"
              checked={captchaStore.is_alpha_numeric}
              onChange={(e) =>
                setCaptchaStore("is_alpha_numeric", e.currentTarget.checked)
              }
            />
          </div>
        </div>
        <div class="captcha-demo-button-wrapper">
          <button
            class="captcha-demo-generate"
            onClick={() => generateCaptcha()}
            disabled={generateCaptcha.loading}
          >
            {generateCaptcha.loading ? "Generating..." : "Generate Captcha"}
          </button>
        </div>
        <Show when={captchaImage()}>
          <div class="captcha-demo-captcha">
            <img src={captchaImage()!} alt="Captcha" />
            <div class="captcha-demo-input">
              <input
                type="text"
                value={answer()}
                onInput={(e) => setAnswer(e.currentTarget.value)}
                placeholder="Enter captcha"
              />
              <button onClick={handleSubmit}>Submit</button>
            </div>
          </div>
        </Show>
      </main>
    </div>
  );
};

export default App;
