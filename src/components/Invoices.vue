<template>
  <div class="page-content">
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-3xl font-bold text-gray-900">Laskut</h1>
        <p class="mt-2 text-sm text-gray-700">
          Hallinnoi jäsenmaksulaskuja
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="openCreateInvoicesModal"
          type="button"
          class="btn btn-success"
        >
          Luo laskuja
        </button>
      </div>
    </div>

    <!-- Suodattimet -->
    <div class="form-card">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-4">
        <div>
          <label class="form-label">Vuosi</label>
          <select
            v-model="filterYear"
            class="form-select"
          >
            <option value="">Kaikki vuodet</option>
            <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
          </select>
        </div>
        <div>
          <label class="form-label">Tila</label>
          <select
            v-model="filterStatus"
            class="form-select"
          >
            <option value="">Kaikki</option>
            <option value="unpaid">Maksamaton</option>
            <option value="paid">Maksettu</option>
          </select>
        </div>
        <div>
          <label class="form-label">Haku</label>
          <input
            v-model="searchTerm"
            type="text"
            placeholder="Viitenumero..."
            class="form-input"
          />
        </div>
      </div>
    </div>

    <!-- Laskutaulukko -->
    <div class="data-table invoices-table">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Viitenumero
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Talous
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Summa
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Luontipäivä
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Eräpäivä
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Tila
            </th>
            <th class="relative px-6 py-3">
              <span class="sr-only">Toiminnot</span>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="invoice in filteredInvoices" :key="invoice.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              {{ invoice.viitenumero }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">{{ invoice.talouden_nimi || 'Nimetön talous' }}</div>
              <div class="text-sm text-gray-500">{{ invoice.osoite }}</div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatCurrency(invoice.summa) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatDate(invoice.luontipaiva) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatDate(invoice.erapaiva) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                class="badge"
                :class="invoice.maksettu ? 'badge-success' : 'badge-warning'"
              >
                {{ invoice.maksettu ? 'Maksettu' : 'Avoin' }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                v-if="!invoice.maksettu"
                @click="markAsPaid(invoice)"
                class="btn btn-sm btn-success mr-2"
              >
                Merkitse maksetuksi
              </button>
              <button
                @click="printInvoice(invoice)"
                class="btn btn-sm btn-outline mr-2"
              >
                Tulosta
              </button>
              <button
                @click="deleteInvoice(invoice)"
                class="btn btn-sm btn-danger"
              >
                Poista
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredInvoices.length === 0" class="text-center py-8">
        <p class="text-gray-500">Laskuja ei löytynyt</p>
      </div>
    </div>

    <!-- Luo laskuja -modaali -->
    <div
      v-if="showCreateModal"
      class="modal-overlay"
      @click="closeCreateModal"
    >
      <div
        class="modal-content"
        @click.stop
      >
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">
            Luo jäsenmaksulaskuja
          </h3>
          
          <form @submit.prevent="createInvoices" class="space-y-4">
            <div>
              <label class="form-label">Vuosi *</label>
              <select
                v-model="invoiceForm.year"
                required
                class="form-select"
              >
                <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
              </select>
            </div>
            
            <div>
              <label class="form-label">Eräpäivä *</label>
              <input
                v-model="invoiceForm.dueDate"
                type="date"
                required
                class="form-input"
              />
            </div>
            
            <div class="bg-yellow-50 border border-yellow-200 rounded-md p-4">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-5 w-5 text-yellow-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M8.257 3.099c.765-1.36 2.722-1.36 3.486 0l5.58 9.92c.75 1.334-.213 2.98-1.742 2.98H4.42c-1.53 0-2.493-1.646-1.743-2.98l5.58-9.92zM11 13a1 1 0 11-2 0 1 1 0 012 0zm-1-8a1 1 0 00-1 1v3a1 1 0 002 0V6a1 1 0 00-1-1z" clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-3">
                  <h3 class="text-sm font-medium text-yellow-800">
                    Huomio
                  </h3>
                  <div class="mt-2 text-sm text-yellow-700">
                    <p>Laskuja luodaan yksi per talous. Samassa taloudessa asuvat jäsenet yhdistetään samalle laskulle.</p>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="closeCreateModal"
                class="btn btn-secondary"
              >
                Peruuta
              </button>
              <button
                type="submit"
                class="btn btn-success"
              >
                Luo laskut
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Tulostusvalinta-modaali -->
    <div
      v-if="showPrintModal"
      class="modal-overlay"
      @click="closePrintModal"
    >
      <div
        class="modal-content"
        @click.stop
      >
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">
            Tulosta lasku {{ selectedInvoice?.viitenumero }}
          </h3>
          
          <p class="text-sm text-gray-600 mb-6">
            Valitse haluatko tulostaa laskun suoraan vai tallentaa sen PDF-tiedostona.
          </p>
          
          <div class="flex justify-end space-x-3">
            <button
              @click="closePrintModal"
              class="btn btn-outline"
            >
              Peruuta
            </button>
            <button
              @click="handleDownloadPDF"
              class="btn btn-secondary"
            >
              Tallenna PDF
            </button>
            <button
              @click="handlePrint"
              class="btn btn-primary"
            >
              Tulosta
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- PDF-komponentti (piilotettu) -->
    <div v-if="showPrintModal" style="position: absolute; left: -9999px; top: -9999px;">
      <InvoicePDF 
        v-if="selectedInvoice && organization"
        ref="pdfComponentRef"
        :invoice="selectedInvoice"
        :organization="organization"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import InvoicePDF from './InvoicePDF.vue'
import { generateAndSavePDF, printInvoice as printInvoiceUtil } from '../utils/pdfGenerator'

interface Invoice {
  id: number
  viitenumero: string
  talouden_nimi?: string
  osoite: string
  summa: number
  luontipaiva: string
  erapaiva: string
  maksettu: boolean
  maksupaiva?: string
}

const invoices = ref<Invoice[]>([])
const searchTerm = ref('')
const filterYear = ref('')
const filterStatus = ref('')
const showCreateModal = ref(false)
const showMarkPaidModal = ref(false)
const selectedInvoice = ref<Invoice | null>(null)
const validationError = ref('')
const showPrintModal = ref(false)
const pdfComponentRef = ref<InstanceType<typeof InvoicePDF> | null>(null)
const organization = ref<any>(null)

const years = computed(() => {
  const currentYear = new Date().getFullYear()
  return Array.from({ length: 5 }, (_, i) => currentYear - 2 + i)
})

const invoiceForm = ref({
  year: new Date().getFullYear(),
  dueDate: new Date(Date.now() + 30 * 24 * 60 * 60 * 1000).toISOString().split('T')[0]
})

const filteredInvoices = computed(() => {
  return invoices.value.filter(invoice => {
    const matchesSearch = !searchTerm.value || 
      invoice.viitenumero.includes(searchTerm.value)
    
    const matchesYear = !filterYear.value || 
      new Date(invoice.luontipaiva).getFullYear().toString() === filterYear.value
    
    const matchesStatus = !filterStatus.value || 
      (filterStatus.value === 'paid' && invoice.maksettu) ||
      (filterStatus.value === 'unpaid' && !invoice.maksettu)
    
    return matchesSearch && matchesYear && matchesStatus
  })
})

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const year = date.getFullYear()
  return `${day}.${month}.${year}`
}

const openCreateInvoicesModal = () => {
  showCreateModal.value = true
}

const closeCreateModal = () => {
  showCreateModal.value = false
}

const createInvoices = async () => {
  try {
    await invoke('create_invoice_for_year', { year: invoiceForm.value.year })
    await loadInvoices()
    closeCreateModal()
    alert('Laskut luotu onnistuneesti!')
  } catch (error) {
    console.error('Virhe luodessa laskuja:', error)
    alert('Virhe luodessa laskuja')
  }
}

const markAsPaid = async (invoice: Invoice) => {
  const paymentDate = prompt('Anna maksupäivä (YYYY-MM-DD):', new Date().toISOString().split('T')[0])
  if (paymentDate) {
    try {
      await invoke('mark_invoice_paid', { 
        id: invoice.id, 
        paymentDate 
      })
      await loadInvoices()
    } catch (error) {
      console.error('Virhe merkitessä laskua maksetuksi:', error)
      alert('Virhe merkitessä laskua maksetuksi')
    }
  }
}

const printInvoice = async (invoice: Invoice) => {
  selectedInvoice.value = invoice
  showPrintModal.value = true
}

const handlePrint = async () => {
  try {
    if (!selectedInvoice.value || !pdfComponentRef.value?.invoiceRef) return
    
    await printInvoiceUtil(pdfComponentRef.value.invoiceRef)
    showPrintModal.value = false
    selectedInvoice.value = null
  } catch (error) {
    console.error('Virhe tulostuksessa:', error)
    alert('Tulostus epäonnistui: ' + error.message)
  }
}

const handleDownloadPDF = async () => {
  try {
    if (!selectedInvoice.value || !pdfComponentRef.value?.invoiceRef) return
    
    const defaultFilename = `lasku_${selectedInvoice.value.viitenumero}.pdf`
    
    await generateAndSavePDF(pdfComponentRef.value.invoiceRef, defaultFilename)
    
    showPrintModal.value = false
    selectedInvoice.value = null
  } catch (error) {
    console.error('Virhe PDF:n luonnissa:', error)
    alert('PDF:n luonti epäonnistui: ' + error.message)
  }
}

const closePrintModal = () => {
  showPrintModal.value = false
  selectedInvoice.value = null
}

const deleteInvoice = async (invoice: Invoice) => {
  if (confirm(`Haluatko varmasti poistaa laskun ${invoice.viitenumero}?`)) {
    try {
      // TODO: Lisää delete_invoice komento
      // await invoke('delete_invoice', { id: invoice.id })
      await loadInvoices()
    } catch (error) {
      console.error('Virhe poistaessa laskua:', error)
      alert('Virhe poistaessa laskua')
    }
  }
}

const loadInvoices = async () => {
  try {
    // TODO: Hae laskut backend:ista
    // invoices.value = await invoke('get_invoices')
    
    // Väliaikainen testidata
    invoices.value = [
      {
        id: 1,
        viitenumero: '202400001',
        talouden_nimi: 'Korhosen perhe',
        osoite: 'Kotikatu 1, 00100 Helsinki',
        summa: 100.00,
        luontipaiva: '2024-01-15',
        erapaiva: '2024-02-15',
        maksettu: false,
        household: {
          talouden_nimi: 'Korhosen perhe',
          vastaanottaja: 'Matti Korhonen'
        },
        address: {
          katuosoite: 'Kotikatu 1',
          postinumero: '00100',
          postitoimipaikka: 'Helsinki'
        },
        lines: [
          {
            line: {
              id: 1,
              kuvaus: 'Jäsenmaksu 2024',
              summa: 50.00
            },
            member: {
              etunimi: 'Matti',
              sukunimi: 'Korhonen'
            }
          },
          {
            line: {
              id: 2,
              kuvaus: 'Jäsenmaksu 2024',
              summa: 50.00
            },
            member: {
              etunimi: 'Liisa',
              sukunimi: 'Korhonen'
            }
          }
        ]
      },
      {
        id: 2,
        viitenumero: '202400002',
        talouden_nimi: 'Virtasen talous',
        osoite: 'Testikatu 2, 00200 Espoo',
        summa: 50.00,
        luontipaiva: '2024-01-15',
        erapaiva: '2024-02-15',
        maksettu: true,
        maksupaiva: '2024-01-20',
        household: {
          talouden_nimi: 'Virtasen talous',
          vastaanottaja: 'Pekka Virtanen'
        },
        address: {
          katuosoite: 'Testikatu 2',
          postinumero: '00200',
          postitoimipaikka: 'Espoo'
        },
        lines: [
          {
            line: {
              id: 3,
              kuvaus: 'Jäsenmaksu 2024',
              summa: 50.00
            },
            member: {
              etunimi: 'Pekka',
              sukunimi: 'Virtanen'
            }
          }
        ]
      },
    ]
  } catch (error) {
    console.error('Virhe ladatessa laskuja:', error)
  }
}

const loadOrganization = async () => {
  try {
    organization.value = await invoke('get_organization')
  } catch (error) {
    console.error('Virhe ladatessa organisaatiota:', error)
  }
}

onMounted(() => {
  loadInvoices()
  loadOrganization()
})
</script>