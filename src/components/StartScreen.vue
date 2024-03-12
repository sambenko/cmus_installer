<template>
    <div class="start-screen-container">
      <div class="start-screen-logo-heading">
        <img :src="logo" alt="Installer Logo" class="logo" />
        <h1>C* Music Player Installer</h1>
      </div>
      <div class="start-screen-button-container">
        <button @click="startProcess">Start</button>
      </div>
    </div>
  </template>
  
  <script>
  import logoImage from '../assets/cmus.jpeg';
  
  export default {
    name: 'StartScreen',
    data() {
      return {
        logo: logoImage,
      };
    },
    methods: {
      startProcess() {
        this.$router.push('/installation');
      },
    },
  };
  </script>
  
  <style scoped>
  .start-screen-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-around;
    height: 100vh;
    padding: 1rem;
    background-color: var(--background-color);
  }
  
  .start-screen-logo-heading {
    align-self: flex-start;
    text-align: center;
    width: 100%;
  }
  
  .start-screen-logo-heading .logo {
    height: 6em;
    padding: 1.5em;
    border-radius: 0;
  }
  
  .start-screen-logo-heading h1 {
    color: var(--color);
    font-size: 2rem;
  }
  
  .start-screen-button-container {
    text-align: center;
    width: 100%;
  }
  
  .start-screen-button-container button {
    padding: 0.75rem 1.5rem;
    font-size: 1.25rem;
    color: #ffffff;
    background-color: #646cff;
    border-radius: 8px;
    border: 1px solid transparent;
    transition: background-color 0.2s ease;
    cursor: pointer;
    outline: none;
  }

  .start-screen-button-container button:active {
  background-color: #4b59d8;
  transform: scale(0.95);
}

.start-screen-button-container button:hover {
  background-color: #5a67d8;
}

.start-screen-button-container button:focus {
  outline: 3px solid #5a67d8;
}
  
  @media (max-width: 600px) {
    .start-screen-logo-heading h1 {
      font-size: 1.75rem;
    }
  }
  </style>
  