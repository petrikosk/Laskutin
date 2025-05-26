<template>
  <div class="invoice-pdf" ref="invoiceRef">
    <!-- Laskun sisältö PDF:ää varten -->
    <div class="invoice-container">
      <!-- Yläosa: Laskuttajan tiedot ja logo -->
      <div class="invoice-header">
        <div class="sender-info">
          <h1 class="company-name">{{ organization?.nimi || 'Yhdistys' }}</h1>
          <div class="address">
            <div>{{ organization?.katuosoite }}</div>
            <div>{{ organization?.postinumero }} {{ organization?.postitoimipaikka }}</div>
          </div>
          <div class="contact-info">
            <div v-if="organization?.puhelinnumero">Puh: {{ organization.puhelinnumero }}</div>
            <div v-if="organization?.sahkoposti">{{ organization.sahkoposti }}</div>
            <div v-if="organization?.y_tunnus">Y-tunnus: {{ organization.y_tunnus }}</div>
          </div>
        </div>
        <div class="invoice-title">
          <h2>LASKU</h2>
        </div>
      </div>

      <!-- Laskun perustiedot -->
      <div class="invoice-details">
        <div class="invoice-info">
          <table class="info-table">
            <tr>
              <td><strong>Laskun numero:</strong></td>
              <td>{{ invoice?.id }}</td>
            </tr>
            <tr>
              <td><strong>Laskun päivämäärä:</strong></td>
              <td>{{ formatDate(invoice?.luontipaiva) }}</td>
            </tr>
            <tr>
              <td><strong>Eräpäivä:</strong></td>
              <td>{{ formatDate(invoice?.erapaiva) }}</td>
            </tr>
            <tr>
              <td><strong>Viitenumero:</strong></td>
              <td>{{ invoice?.viitenumero }}</td>
            </tr>
          </table>
        </div>
        
        <!-- Vastaanottajan tiedot -->
        <div class="recipient-info">
          <h3>Laskun saaja</h3>
          <div class="recipient-address">
            <div><strong>{{ invoice?.household?.vastaanottaja || invoice?.household?.talouden_nimi || invoice?.talouden_nimi || 'Nimetön vastaanottaja' }}</strong></div>
            <div>{{ invoice?.address?.katuosoite }}</div>
            <div>{{ invoice?.address?.postinumero }} {{ invoice?.address?.postitoimipaikka }}</div>
          </div>
        </div>
      </div>

      <!-- Laskurivit -->
      <div class="invoice-lines">
        <h3>Laskutettavat erät</h3>
        <table class="lines-table">
          <thead>
            <tr>
              <th>Kuvaus</th>
              <th>Jäsen</th>
              <th class="amount">Summa</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="line in invoice?.lines" :key="line.line.id">
              <td>{{ line.line.kuvaus }}</td>
              <td>{{ line.member.etunimi }} {{ line.member.sukunimi }}</td>
              <td class="amount">{{ formatCurrency(line.line.summa) }}</td>
            </tr>
          </tbody>
          <tfoot>
            <tr class="total-row">
              <td colspan="2"><strong>Yhteensä:</strong></td>
              <td class="amount"><strong>{{ formatCurrency(invoice?.summa) }}</strong></td>
            </tr>
          </tfoot>
        </table>
      </div>

      <!-- Maksuohjeet -->
      <div class="payment-info">
        <h3>Maksutiedot</h3>
        <table class="payment-table">
          <tr>
            <td><strong>Saajan tilinumero:</strong></td>
            <td>{{ organization?.pankkitili || 'FI00 0000 0000 0000 00' }}</td>
          </tr>
          <tr v-if="organization?.bic">
            <td><strong>BIC:</strong></td>
            <td>{{ organization.bic }}</td>
          </tr>
          <tr>
            <td><strong>Viitenumero:</strong></td>
            <td>{{ invoice?.viitenumero }}</td>
          </tr>
          <tr>
            <td><strong>Eräpäivä:</strong></td>
            <td>{{ formatDate(invoice?.erapaiva) }}</td>
          </tr>
          <tr>
            <td><strong>Maksettava summa:</strong></td>
            <td><strong>{{ formatCurrency(invoice?.summa) }}</strong></td>
          </tr>
        </table>
      </div>

      <!-- Pankkiviivakoodi -->
      <div class="barcode-section">
        <div class="barcode-label">Pankkiviivakoodi:</div>
        <svg ref="barcodeRef" class="barcode-svg"></svg>
        <!-- Fallback jos SVG-viivakoodi ei toimi -->
        <div v-if="!barcodeRef" class="simple-barcode">
          <div class="barcode-lines">||||||||||||||||||||||||||||||||||||||||</div>
        </div>
      </div>

      <!-- Lisätiedot -->
      <div class="additional-info">
        <p>Kiitos jäsenyydestäsi!</p>
        <p>Laskua koskevissa kysymyksissä ottakaa yhteyttä: {{ organization?.sahkoposti || organization?.puhelinnumero }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, nextTick, watch } from 'vue'
import JsBarcode from 'jsbarcode'

interface Props {
  invoice: any
  organization: any
}

const props = defineProps<Props>()
const invoiceRef = ref<HTMLElement>()
const barcodeRef = ref<SVGElement>()

// Debug: tulosta laskudata konsoliin (voi poistaa myöhemmin)
// console.log('Invoice data:', props.invoice)
// console.log('Invoice lines:', props.invoice?.lines)

// Luo viivakoodi komponentti ladattaessa
const createBarcode = () => {
  if (barcodeRef.value) {
    try {
      const barcodeData = generateBarcode()
      console.log('Generating barcode for:', barcodeData)
      
      // Käytä numerosarjaa sellaisenaan - pankkireferenssi
      JsBarcode(barcodeRef.value, barcodeData, {
        format: "CODE128",
        width: 2,
        height: 40,
        displayValue: false, // Ei tekstiä viivakoodin alla
        margin: 0,
        text: "", // Varmista että ei ole tekstiä
        textMargin: 0,
        fontSize: 0
      })
      
      // Poista kaikki text-elementit varmistukseksi
      const textElements = barcodeRef.value.querySelectorAll('text')
      textElements.forEach(el => el.remove())
    } catch (error) {
      console.error('Virhe viivakoodin luonnissa:', error)
      // Fallback: näytä vain numeroina
      if (barcodeRef.value) {
        barcodeRef.value.innerHTML = `<text x="50%" y="50%" text-anchor="middle" font-family="monospace">${generateBarcode()}</text>`
      }
    }
  }
}

onMounted(() => {
  nextTick(() => {
    createBarcode()
  })
})

// Päivitä viivakoodi kun laskudata muuttuu
watch(() => props.invoice, () => {
  nextTick(() => {
    createBarcode()
  })
}, { deep: true })

const formatDate = (dateString: string) => {
  if (!dateString) return ''
  return new Date(dateString).toLocaleDateString('fi-FI')
}

const formatCurrency = (amount: number) => {
  if (typeof amount !== 'number') return '0,00 €'
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}

// Pankkiviivakoodin generointi (yksinkertainen versio)
const generateBarcode = () => {
  if (!props.invoice || !props.organization) return ''
  
  const tilinumero = (props.organization.pankkitili || 'FI0000000000000000').replace(/\s/g, '')
  const summa = Math.round((props.invoice.summa || 0) * 100).toString().padStart(8, '0')
  const viitenumero = (props.invoice.viitenumero || '').padStart(20, '0')
  const erapaiva = props.invoice.erapaiva ? 
    props.invoice.erapaiva.replace(/-/g, '').slice(2) : '000000' // YYMMDD
  
  // Yksinkertainen versio pankkiviivakoodista
  return `4${tilinumero}${summa}${viitenumero}${erapaiva}`
}


defineExpose({
  invoiceRef
})
</script>

<style scoped>
.invoice-pdf {
  background: white;
  color: black;
  font-family: Arial, sans-serif;
  font-size: 12px;
  line-height: 1.4;
}

.invoice-container {
  width: 210mm;
  max-height: 250mm;
  margin: 0;
  padding: 20mm;
  box-sizing: border-box;
  transform-origin: top left;
}

.invoice-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 30px;
  border-bottom: 2px solid #333;
  padding-bottom: 20px;
}

.company-name {
  font-size: 24px;
  font-weight: bold;
  margin: 0 0 10px 0;
  color: #333;
}

.address {
  margin-bottom: 10px;
}

.contact-info {
  font-size: 11px;
  color: #666;
}

.invoice-title h2 {
  font-size: 28px;
  font-weight: bold;
  margin: 0;
  color: #333;
}

.invoice-details {
  display: flex;
  justify-content: space-between;
  margin-bottom: 30px;
}

.info-table,
.payment-table {
  border-collapse: collapse;
}

.info-table td,
.payment-table td {
  padding: 5px 10px 5px 0;
  vertical-align: top;
}

.recipient-info {
  text-align: right;
}

.recipient-info h3 {
  margin: 0 0 10px 0;
  font-size: 14px;
}

.recipient-address {
  border: 1px solid #ccc;
  padding: 15px;
  background: #f9f9f9;
  min-width: 200px;
}

.invoice-lines {
  margin-bottom: 30px;
}

.invoice-lines h3 {
  margin: 0 0 15px 0;
  font-size: 16px;
  border-bottom: 1px solid #ccc;
  padding-bottom: 5px;
}

.lines-table {
  width: 100%;
  border-collapse: collapse;
  border: 1px solid #ccc;
}

.lines-table th,
.lines-table td {
  border: 1px solid #ccc;
  padding: 8px;
  text-align: left;
}

.lines-table th {
  background: #f5f5f5;
  font-weight: bold;
}

.lines-table .amount {
  text-align: right;
  width: 100px;
}

.total-row {
  background: #f0f0f0;
  font-weight: bold;
}

.payment-info {
  margin-bottom: 30px;
}

.payment-info h3 {
  margin: 0 0 15px 0;
  font-size: 16px;
  border-bottom: 1px solid #ccc;
  padding-bottom: 5px;
}

.barcode-section {
  margin-bottom: 30px;
  text-align: center;
  border: 1px solid #ccc;
  padding: 15px;
  background: #f9f9f9;
  overflow: hidden;
}

.barcode-label {
  font-weight: bold;
  margin-bottom: 10px;
}

.barcode-svg {
  max-width: 100%;
  height: 50px;
  margin: 5px 0;
  display: block;
}

.barcode-svg text {
  display: none !important;
}

.simple-barcode {
  margin: 10px 0;
}

.barcode-lines {
  font-family: 'Courier New', monospace;
  font-size: 14px;
  letter-spacing: 1px;
  line-height: 1;
  background: #000;
  color: #fff;
  padding: 10px 5px;
  text-align: center;
}


.additional-info {
  text-align: center;
  color: #666;
  font-size: 11px;
  margin-top: 30px;
}

.additional-info p {
  margin: 5px 0;
}

/* Tulostustyylitys */
@media print {
  .invoice-pdf {
    font-size: 11px;
  }
  
  .invoice-container {
    padding: 15mm;
  }
  
  .company-name {
    font-size: 20px;
  }
  
  .invoice-title h2 {
    font-size: 24px;
  }
}
</style>