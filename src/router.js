import { createRouter, createWebHashHistory } from 'vue-router';
import App from './App.vue';
import StartScreen from './components/StartScreen.vue';
import InstallationScreen from './components/InstallationScreen.vue';
import InstallationProgress from './components/InstallationProgress.vue';

const routes = [
  {
    path: '/',
    component: App,
    children: [
      { path: '', name: "StartScreen", component: StartScreen },
      { path: 'installation', name: "Installation", component: InstallationScreen },
      { path: '/installation-progress/:version/:path', name: "installation-progress", component: InstallationProgress, props: true },
    ],
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;