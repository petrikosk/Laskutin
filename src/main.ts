import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./style.css";

// Import Vue Datepicker
import VueDatePicker from '@vuepic/vue-datepicker';
import '@vuepic/vue-datepicker/dist/main.css';

// Import components
import Dashboard from "./components/Dashboard.vue";
import Members from "./components/Members.vue";
import Invoices from "./components/Invoices.vue";
import MembershipFees from "./components/MembershipFees.vue";
import OrganizationSettings from "./components/OrganizationSettings.vue";

const routes = [
  { path: "/", component: Dashboard, name: "dashboard" },
  { path: "/members", component: Members, name: "members" },
  { path: "/invoices", component: Invoices, name: "invoices" },
  { path: "/fees", component: MembershipFees, name: "fees" },
  { path: "/settings", component: OrganizationSettings, name: "settings" },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

const pinia = createPinia();

createApp(App)
  .use(router)
  .use(pinia)
  .component('VueDatePicker', VueDatePicker)
  .mount("#app");
