<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { listen } from "@tauri-apps/api/event";

  let prompt = "";
  let response = "";
  let isLoading = false;

  // Listen for streaming responses from the Rust backend
  listen("ollama-response", (event) => {
    response += event.payload.response;
    if (event.payload.done) {
      isLoading = false;
    }
  });

  async function submitPrompt() {
    if (!prompt || isLoading) return;
    isLoading = true;
    response = ""; // Clear previous response
    await invoke("invoke_ollama", { prompt });
  }
</script>

<main class="container">
  <h1>Ollama Chat</h1>

  <textarea
    bind:value={prompt}
    placeholder="Ask your local LLM anything..."
    on:keydown={(e) => {
      if (e.key === 'Enter' && !e.shiftKey) {
        e.preventDefault();
        submitPrompt();
      }
    }}
  ></textarea>
  <button on:click={submitPrompt} disabled={isLoading}>
    {isLoading ? "Thinking..." : "Send"}
  </button>

  {#if response}
    <div class="response">
      <pre>{response}</pre>
    </div>
  {/if}
</main>