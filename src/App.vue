<template>
  <div class="min-h-screen bg-gray-50">
    <!-- Navigaatio -->
    <nav class="bg-white shadow-sm border-b border-gray-200">
      <div class="nav-container">
        <div class="nav-flex">
          <div class="flex">
            <div class="nav-brand">
              <h1 class="nav-title">Laskutin</h1>
            </div>
            <div class="nav-links">
              <router-link
                v-for="item in navigation"
                :key="item.name"
                :to="item.href"
                class="nav-link"
                :class="{ 'active': route.name === item.name }"
              >
                {{ item.label }}
              </router-link>
            </div>
          </div>
        </div>
      </div>
    </nav>

    <!-- Sisältö -->
    <main class="main-content">
      <router-view />
    </main>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const navigation = [
  { name: 'dashboard', label: 'Etusivu', href: '/' },
  { name: 'members', label: 'Jäsenet', href: '/members' },
  { name: 'households', label: 'Taloudet', href: '/households' },
  { name: 'invoices', label: 'Laskut', href: '/invoices' },
  { name: 'fees', label: 'Jäsenmaksut', href: '/fees' },
  { name: 'settings', label: 'Asetukset', href: '/settings' },
]
</script>