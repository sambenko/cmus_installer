<template>
  <div id="app">
    <router-view />
  </div>
</template>

<script>
export default {
  name: 'App',
  methods: {
    showInstallationScreen() {
      this.$router.push({ name: 'Installation' });
    },
    showProgressScreen(version, path) {
      this.$router.push({
        name: 'Progress',
        query: { version, path },
      });
    },
    installationComplete() {
      this.$router.push('/');
    },
  },
};
</script>


<style>
body,
html,
#app {
  overflow: hidden;
  height: 100%;
  margin: 0;
  font-family: Arial, sans-serif;
}
</style>
