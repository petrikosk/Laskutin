<template>
  <div>
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900">Etusivu</h1>
      <p class="mt-2 text-sm text-gray-600">
        Tervetuloa jäsenmaksulaskutusohjelmaan
      </p>
    </div>

    <!-- Tilastokortit -->
    <div class="grid grid-cols-1 gap-5 sm:grid-cols-2 lg:grid-cols-4 mb-8">
      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="icon icon-lg icon-group text-blue-600"></div>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Jäsenet yhteensä
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ stats.total_members }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="icon icon-lg icon-document text-yellow-600"></div>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Avoimet laskut
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ stats.open_invoices }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="icon icon-lg icon-euro text-red-600"></div>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Saamiset yhteensä
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ formatCurrency(stats.total_receivables) }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <div class="icon icon-lg icon-chart text-green-600"></div>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Tämän vuoden tulot
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ formatCurrency(stats.yearly_income) }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Toiminnot -->
    <div class="action-cards-container">
      <div class="action-cards-content">
        <h3 class="action-cards-title">
          Pikatoiminnot
        </h3>
        <div class="action-cards-grid">
          <router-link to="/members" class="action-card">
            <div class="action-card-icon">
              <div class="icon icon-lg icon-group text-blue-600"></div>
            </div>
            <div class="action-card-text">
              <p class="action-card-title">Hallinnoi jäseniä</p>
              <p class="action-card-description">Lisää, muokkaa ja poista jäseniä</p>
            </div>
          </router-link>

          <router-link to="/invoices" class="action-card">
            <div class="action-card-icon">
              <div class="icon icon-lg icon-invoice text-green-600"></div>
            </div>
            <div class="action-card-text">
              <p class="action-card-title">Luo laskuja</p>
              <p class="action-card-description">Generoi jäsenmaksulaskuja</p>
            </div>
          </router-link>

          <router-link to="/fees" class="action-card">
            <div class="action-card-icon">
              <div class="icon icon-lg icon-euro text-purple-600"></div>
            </div>
            <div class="action-card-text">
              <p class="action-card-title">Jäsenmaksut</p>
              <p class="action-card-description">Määrittele vuosittaiset maksut</p>
            </div>
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Stats {
  total_members: number
  open_invoices: number
  total_receivables: number
  yearly_income: number
}

const stats = ref<Stats>({
  total_members: 0,
  open_invoices: 0,
  total_receivables: 0,
  yearly_income: 0,
})

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}

const loadStats = async () => {
  try {
    const dashboardStats = await invoke('get_dashboard_stats') as Stats
    stats.value = dashboardStats
  } catch (error) {
    console.error('Virhe ladatessa tilastoja:', error)
    // Fallback data if backend fails
    stats.value = {
      total_members: 0,
      open_invoices: 0,
      total_receivables: 0,
      yearly_income: 0,
    }
  }
}

onMounted(() => {
  loadStats()
})
</script>

<style scoped>
.stat-card {
  @apply bg-white p-6 rounded-lg shadow-md border border-gray-200;
}

.action-cards-container {
  @apply bg-white rounded-xl shadow-lg border border-gray-200 overflow-hidden;
}

.action-cards-content {
  @apply p-6;
}

.action-cards-title {
  @apply text-lg font-bold text-gray-900 mb-6;
}

.action-cards-grid {
  @apply grid gap-4;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
}

.action-card {
  @apply p-6 bg-gradient-to-br from-white to-gray-50 border-2 border-gray-200 rounded-2xl flex items-center gap-4 no-underline text-inherit transition-all duration-200 shadow-md hover:shadow-lg hover:border-gray-300 hover:-translate-y-1;
}

.action-card-icon {
  @apply flex-shrink-0;
}

.action-card-text {
  @apply flex-1;
}

.action-card-title {
  @apply font-semibold text-gray-900 mb-1;
}

.action-card-description {
  @apply text-sm text-gray-600;
}
</style>