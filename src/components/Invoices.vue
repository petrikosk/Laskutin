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
              <DateInput
                v-model="invoiceForm.dueDate"
                :required="true"
                placeholder="Valitse eräpäivä"
                input-class="form-input"
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

    <!-- Varmistusdialogie -->
    <ConfirmDialog
      :show="confirmDialog.show"
      :title="confirmDialog.title"
      :message="confirmDialog.message"
      :type="confirmDialog.type"
      :icon="confirmDialog.icon"
      :confirm-text="confirmDialog.confirmText"
      :cancel-text="confirmDialog.cancelText"
      @confirm="confirmDialog.onConfirm"
      @cancel="closeConfirmDialog"
    />

    <!-- Maksu-dialogie -->
    <PaymentDialog
      :show="showPaymentDialog"
      :invoice-reference="selectedInvoiceForPayment?.viitenumero"
      @confirm="handlePaymentConfirm"
      @cancel="closePaymentDialog"
    />

    <!-- Onnistumis-ilmoitus -->
    <SuccessNotification
      :show="successNotification.show"
      :title="successNotification.title"
      :message="successNotification.message"
      @close="closeSuccessNotification"
    />

    <!-- Virhe dialogi -->
    <AlertDialog
      :show="showErrorDialog"
      title="Virhe"
      :message="errorMessage"
      type="error"
      icon="error"
      @close="showErrorDialog = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import InvoicePDF from './InvoicePDF.vue'
import ConfirmDialog from './ConfirmDialog.vue'
import DateInput from './DateInput.vue'
import PaymentDialog from './PaymentDialog.vue'
import SuccessNotification from './SuccessNotification.vue'
import AlertDialog from './AlertDialog.vue'
import { generateAndSavePDF, printInvoice as printInvoiceUtil } from '../utils/pdfGenerator'
import { formatDate } from '../utils/dateUtils'

interface Invoice {
  id: number
  viitenumero: string
  laskunumero?: string
  talouden_nimi?: string
  vastaanottaja?: string
  osoite: string
  summa: number
  luontipaiva: string
  erapaiva: string
  maksettu: boolean
  maksupaiva?: string
  household?: any
  address?: any
  lines?: any[]
}

const invoices = ref<Invoice[]>([])
const searchTerm = ref('')
const filterYear = ref('')
const filterStatus = ref('')
const showCreateModal = ref(false)
const selectedInvoice = ref<Invoice | null>(null)
const showPrintModal = ref(false)
const pdfComponentRef = ref<InstanceType<typeof InvoicePDF> | null>(null)
const organization = ref<any>(null)
const showPaymentDialog = ref(false)
const selectedInvoiceForPayment = ref<Invoice | null>(null)
const showErrorDialog = ref(false)
const errorMessage = ref('')

const successNotification = ref({
  show: false,
  title: '',
  message: ''
})

const confirmDialog = ref({
  show: false,
  title: '',
  message: '',
  type: 'warning' as 'warning' | 'danger' | 'info',
  icon: 'warning' as 'warning' | 'danger' | 'info',
  confirmText: 'Kyllä',
  cancelText: 'Peruuta',
  onConfirm: () => {}
})

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


const openCreateInvoicesModal = () => {
  showCreateModal.value = true
}

const closeCreateModal = () => {
  showCreateModal.value = false
}

const showConfirmDialog = (options: {
  title: string
  message: string
  type?: 'warning' | 'danger' | 'info'
  confirmText?: string
  cancelText?: string
  onConfirm: () => void
}) => {
  confirmDialog.value = {
    show: true,
    title: options.title,
    message: options.message,
    type: options.type || 'warning',
    icon: options.type || 'warning',
    confirmText: options.confirmText || 'Kyllä',
    cancelText: options.cancelText || 'Peruuta',
    onConfirm: () => {
      options.onConfirm()
      closeConfirmDialog()
    }
  }
}

const closeConfirmDialog = () => {
  confirmDialog.value.show = false
}

const createInvoices = async () => {
  try {
    // Validoi ensin ennen laskujen luontia
    const validationMessage = await invoke('validate_invoice_creation', { year: invoiceForm.value.year })
    
    // Kysy varmistus käyttäjältä ENNEN mitään toimintoja
    showConfirmDialog({
      title: 'Vahvista laskujen luonti',
      message: `${validationMessage}\n\nHaluatko jatkaa laskujen luontia?`,
      type: 'warning',
      confirmText: 'Luo laskut',
      cancelText: 'Peruuta',
      onConfirm: async () => {
        try {
          // Luo laskut vasta vahvistuksen jälkeen
          const createdInvoices = await invoke('create_invoice_for_year', { year: invoiceForm.value.year }) as any[]
          await loadInvoices()
          closeCreateModal()
          showSuccessNotification(
            'Laskut luotu onnistuneesti!',
            `${createdInvoices.length} laskua luotiin vuodelle ${invoiceForm.value.year}.`
          )
        } catch (error: unknown) {
          console.error('Virhe luodessa laskuja:', error)
          errorMessage.value = 'Virhe luodessa laskuja: ' + String(error)
          showErrorDialog.value = true
        }
      }
    })
  } catch (error: unknown) {
    console.error('Virhe luodessa laskuja:', error)
    errorMessage.value = 'Virhe luodessa laskuja: ' + String(error)
    showErrorDialog.value = true
  }
}

const markAsPaid = async (invoice: Invoice) => {
  selectedInvoiceForPayment.value = invoice
  showPaymentDialog.value = true
}

const handlePaymentConfirm = async (paymentDate: string) => {
  if (!selectedInvoiceForPayment.value) return
  
  try {
    await invoke('mark_invoice_paid', { 
      id: selectedInvoiceForPayment.value.id, 
      paymentDate 
    })
    
    // Tallenna viitenumero ennen dialogin sulkemista
    const viitenumero = selectedInvoiceForPayment.value.viitenumero
    
    await loadInvoices()
    closePaymentDialog()
    showSuccessNotification(
      'Lasku merkitty maksetuksi!',
      `Lasku ${viitenumero} on merkitty maksetuksi ${paymentDate}.`
    )
  } catch (error: unknown) {
    console.error('Virhe merkitessä laskua maksetuksi:', error)
    errorMessage.value = 'Virhe merkitessä laskua maksetuksi: ' + String(error)
    showErrorDialog.value = true
  }
}

const closePaymentDialog = () => {
  showPaymentDialog.value = false
  selectedInvoiceForPayment.value = null
}

const showSuccessNotification = (title: string, message: string) => {
  successNotification.value = {
    show: true,
    title,
    message
  }
}

const closeSuccessNotification = () => {
  successNotification.value.show = false
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
  } catch (error: unknown) {
    console.error('Virhe tulostuksessa:', error)
    errorMessage.value = 'Tulostus epäonnistui: ' + (error instanceof Error ? error.message : String(error))
    showErrorDialog.value = true
  }
}

const handleDownloadPDF = async () => {
  try {
    if (!selectedInvoice.value || !pdfComponentRef.value?.invoiceRef) return
    
    const defaultFilename = `lasku_${selectedInvoice.value.viitenumero}.pdf`
    
    await generateAndSavePDF(pdfComponentRef.value.invoiceRef, defaultFilename)
    
    showPrintModal.value = false
    selectedInvoice.value = null
  } catch (error: unknown) {
    console.error('Virhe PDF:n luonnissa:', error)
    errorMessage.value = 'PDF:n luonti epäonnistui: ' + (error instanceof Error ? error.message : String(error))
    showErrorDialog.value = true
  }
}

const closePrintModal = () => {
  showPrintModal.value = false
  selectedInvoice.value = null
}

const deleteInvoice = async (invoice: Invoice) => {
  // Kysy varmistus ENNEN toimintoa
  showConfirmDialog({
    title: 'Poista lasku',
    message: `Haluatko varmasti poistaa laskun ${invoice.viitenumero}?`,
    type: 'danger',
    confirmText: 'Poista',
    cancelText: 'Peruuta',
    onConfirm: async () => {
      try {
        await invoke('delete_invoice', { id: invoice.id })
        await loadInvoices()
      } catch (error) {
        console.error('Virhe poistaessa laskua:', error)
        errorMessage.value = 'Virhe poistaessa laskua'
        showErrorDialog.value = true
      }
    }
  })
}

const loadInvoices = async () => {
  try {
    const invoiceData = await invoke('get_invoices') as any[]
    // Muunna backend-data sopivaan muotoon frontend:lle
    invoices.value = invoiceData.map((item: any) => ({
      id: item.invoice.id,
      viitenumero: item.invoice.viitenumero,
      laskunumero: item.invoice.laskunumero,
      talouden_nimi: item.household.talouden_nimi,
      vastaanottaja: item.household.vastaanottaja,
      osoite: `${item.address.katuosoite}, ${item.address.postinumero} ${item.address.postitoimipaikka}`,
      summa: item.invoice.summa,
      luontipaiva: item.invoice.luontipaiva,
      erapaiva: item.invoice.erapaiva,
      maksettu: item.invoice.maksettu,
      maksupaiva: item.invoice.maksupaiva,
      // Säilytä koko data-objekti PDF:ää varten
      household: item.household,
      address: item.address,
      lines: item.lines
    }))
  } catch (error) {
    console.error('Virhe ladatessa laskuja:', error)
    // Fallback: tyhjä lista
    invoices.value = []
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