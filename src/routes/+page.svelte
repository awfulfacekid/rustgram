<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import './styles.css';

  let token: string = "";
  let greeting: string = "";
  let buttonText: string = "";
  let buttonHandler: string = "";
  let buttons: Array<{ text: string; handler: string }> = [];
  let message: string = "";

  function addButton() {
    if (buttonText && buttonHandler) {
      buttons = [...buttons, { text: buttonText, handler: buttonHandler }];
      buttonText = "";
      buttonHandler = "";
    }
  }

  async function startBot() {
    try {
      await invoke("start_bot", {
        token,
        greeting,
        buttons
      });
      message = "Бот запущен, rustgram @awfulfacekid";
    } catch (error) {
      message = `Error starting bot: ${error}`;
    }
  }

  async function stopBot() {
    try {
      await invoke("stop_bot");
      message = "Bot stopped successfully!";
    } catch (error) {
      message = `Error stopping bot: ${error}`;
    }
  }
</script>

<div class="container">
  <h1>RustGramUI (@awfulfacekid)</h1>

  <form on:submit|preventDefault={startBot}>
    <label for="token">Бот токен:</label>
    <input id="token" bind:value={token} placeholder="Введите ваш токен (@Botfather)" />
    
    <label for="greeting">Приветствие:</label>
    <input id="greeting" bind:value={greeting} placeholder="Введите сообщение которое будет показано при /start" />
    
    <div class="button-form">
      <input
        id="buttonText"
        bind:value={buttonText}
        placeholder="Текст кнопки"
      />
      <input
        id="buttonHandler"
        bind:value={buttonHandler}
        placeholder="Обработчик кнопки"
      />
      <button type="button" on:click={addButton} id="addbuttonuncs">Добавить кнопку</button>
    </div>

    <div class="button-preview">
      {#each buttons as { text, handler }}
        <div class="button-item">
          <span>{text}</span>
          <span>{handler}</span>
        </div>
      {/each}
    </div>

    <div class="button-container">
      <button type="submit" class="start">Включить</button>
      <button on:click={stopBot} class="stop">Выключить</button>
    </div>
  </form>

  <p>{message}</p>
</div>

<style>
  .button-form {
    display: flex;
    gap: 10px;
    margin-bottom: 10px;
  }

  .button-preview {
    margin-bottom: 20px;
  }

  .button-item {
    display: flex;
    justify-content: space-between;
    padding: 5px;
    border: 1px solid #ccc;
    border-radius: 4px;
    margin-bottom: 5px;
  }

  .button-item span {
    flex: 1;
  }
</style>
