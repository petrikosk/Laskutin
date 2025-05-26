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
            <svg class="icon-lg text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z" />
            </svg>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Jäsenet yhteensä
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ stats.totalMembers }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <svg class="icon-lg text-yellow-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
            </svg>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Avoimet laskut
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ stats.openInvoices }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <svg class="icon-lg text-red-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
            </svg>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Saamiset yhteensä
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ formatCurrency(stats.totalReceivables) }}
            </p>
          </div>
        </div>
      </div>

      <div class="stat-card">
        <div class="flex items-center">
          <div class="flex-shrink-0">
            <svg class="icon-lg text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v4a2 2 0 01-2 2h-2a2 2 0 00-2-2z" />
            </svg>
          </div>
          <div class="ml-4 flex-1">
            <p class="text-sm font-medium text-gray-600">
              Tämän vuoden tulot
            </p>
            <p class="text-2xl font-bold text-gray-900">
              {{ formatCurrency(stats.yearlyIncome) }}
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- Toiminnot -->
    <div class="card">
      <div class="p-6">
        <h3 class="text-lg font-bold text-gray-900 mb-6">
          Pikatoiminnot
        </h3>
        <div class="grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3">
          <router-link to="/members" class="action-card">
            <div class="flex-shrink-0">
              <svg class="icon-lg text-blue-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4.354a4 4 0 110 5.292M15 21H3v-1a6 6 0 0112 0v1zm0 0h6v-1a6 6 0 00-9-5.197m13.5-9a2.25 2.25 0 11-4.5 0 2.25 2.25 0 014.5 0z" />
              </svg>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-gray-900">Hallinnoi jäseniä</p>
              <p class="text-sm text-gray-600">Lisää, muokkaa ja poista jäseniä</p>
            </div>
          </router-link>

          <router-link to="/invoices" class="action-card">
            <div class="flex-shrink-0">
              <svg class="icon-lg text-green-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-gray-900">Luo laskuja</p>
              <p class="text-sm text-gray-600">Generoi jäsenmaksulaskuja</p>
            </div>
          </router-link>

          <router-link to="/fees" class="action-card">
            <div class="flex-shrink-0">
              <svg class="icon-lg text-purple-600" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1" />
              </svg>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-gray-900">Jäsenmaksut</p>
              <p class="text-sm text-gray-600">Määrittele vuosittaiset maksut</p>
            </div>
          </router-link>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'

const stats = ref({
  totalMembers: 0,
  openInvoices: 0,
  totalReceivables: 0,
  yearlyIncome: 0,
})

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}

onMounted(async () => {
  // TODO: Hae tilastot backend:ista
  stats.value = {
    totalMembers: 45,
    openInvoices: 12,
    totalReceivables: 2450.00,
    yearlyIncome: 8750.00,
  }
})
</script>