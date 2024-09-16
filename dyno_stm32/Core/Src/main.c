/**
 ******************************************************************************
 * @file           : main.c
 * @brief          : Main program body
 ******************************************************************************
 * Copyright (c) 2023 Rizal Achmad Pahlevi
 ******************************************************************************
 */

/* Includes ------------------------------------------------------------------*/
#include "main.h"
#include "stm32f4xx_hal_tim.h"
#include "usb_device.h"
#include "usbd_cdc_if.h"

__IO uint32_t v_pulse_rpm = 0;
__IO uint32_t v_last_time = 0;

SPI_HandleTypeDef hspi1;
SPI_HandleTypeDef hspi_max6675;
TIM_HandleTypeDef htim4;

#define DYNO_IMPLEMENTATION
#include <dyno.h>

/* Private function prototypes -----------------------------------------------*/
void SystemClock_Config(void);
static void MX_GPIO_Init(void);
static void MX_TIM4_Init(void);
static void MX_SPI1_Init(void);
static void MX_NVIC_Init(void);
static void MX_ENCODER_Init(void);

/**
 * @brief  The application entry point.
 * @retval int
 */
int main(void) {
    /* Reset of all peripherals, Initializes the Flash interface and the Systick. */
    HAL_Init();

    /* Configure the system clock */
    SystemClock_Config();

    /* Initialize all configured peripherals */
    MX_GPIO_Init();
    MX_TIM4_Init();
    MX_USB_DEVICE_Init();
    MX_SPI1_Init();
    MX_ENCODER_Init();

    /* Initialize interrupts */
    MX_NVIC_Init();
    datadyno_reset(&dyno);
    uint32_t last_tick = 0;
    while (1) {
        // check if the `v_usb_send_bit` is set by interrupt, for time to sending
        // data and check if CDC USBDevice Driver is ready and opened
        uint32_t ticks = HAL_GetTick() - last_tick;
        if ((ticks >= PERIOD_SEND_DATA_MS) && CDC_IS_USB_OPENED(&hUsbDeviceFS)) {
            last_tick = HAL_GetTick();
            // check first if counter is CW ( the required direction ) ignoring the CCW
            // the ccw value  in 16 bit is negative
            if (TIM2->CNT < INT16_MAX)
                dyno.pulse_enc = TIM2->CNT;
            TIM2->CNT = 0;
#ifdef WITH_PHASE_Z
            dyno.pulse_enc_z = v_pulse_enc;
#else
            dyno.pulse_enc_z = dyno.pulse_enc / MAX_PPR_ENCODER;
#endif
            dyno.pulse_rpm = v_pulse_rpm;
            v_pulse_rpm = 0;

            dyno.period = ticks;

            /// send data to usb com
            memcpy(&BUFFER_DATA[0], &dyno, DYNO_SIZE_DATA);
            CDC_Transmit_FS(&BUFFER_DATA[0], BUFFER_DATA_SIZE);

            MAX6675_Temp(&dyno.temperature);
        }
    }
}

// start the process
void MX_Start(void) {
    HAL_TIM_IC_Start_IT(&htim4, TIM_CHANNEL_4);
    SET_PIN(LED_INDICATOR_Port, LED_INDICATORA_Pin);
}

// stop the process
void MX_Stop(void) {
    datadyno_reset(&dyno);
    HAL_TIM_IC_Stop_IT(&htim4, TIM_CHANNEL_4);
    RESET_PIN(LED_INDICATOR_Port, LED_INDICATORA_Pin);
}

static void MX_ENCODER_Init(void) {
    // clang-format off
    // configure TIM2 as Encoder input
    __HAL_RCC_TIM2_CLK_ENABLE();

    TIM2->CR1   = 0x0001;  // CEN(Counter ENable)='1'     < TIM control register 1
    TIM2->SMCR  = 0x0003;  // SMS='011' (Encoder mode 3)  < TIM slave mode control register
    TIM2->CCMR1 = 0xF1F1;  // CC1S='01' CC2S='01'         < TIM capture/compare mode register 1
    TIM2->CCMR2 = 0x0000;  //                             < TIM capture/compare mode register 2
    TIM2->CCER  = 0x0011;  // CC1P CC2P                   < TIM capture/compare enable register
    TIM2->PSC   = 0x0000;  // Prescaler = (0+1)           < TIM prescaler
    TIM2->ARR   = 0xFFFF;  // reload at 0xfffffff         < TIM auto-reload register

    TIM2->CNT = 0x0000;    //reset the counter before we use it
    // clang-format on
}

/**
 * @brief System Clock Configuration
 * @retval None
 */
void SystemClock_Config(void) {
    RCC_OscInitTypeDef RCC_OscInitStruct = {0};
    RCC_ClkInitTypeDef RCC_ClkInitStruct = {0};

    /** Configure the main internal regulator output voltage
     */
    __HAL_RCC_PWR_CLK_ENABLE();
    __HAL_PWR_VOLTAGESCALING_CONFIG(PWR_REGULATOR_VOLTAGE_SCALE1);

    /** Initializes the RCC Oscillators according to the specified parameters
     * in the RCC_OscInitTypeDef structure.
     */
    RCC_OscInitStruct.OscillatorType = RCC_OSCILLATORTYPE_HSE;
    RCC_OscInitStruct.HSEState = RCC_HSE_ON;
    RCC_OscInitStruct.PLL.PLLState = RCC_PLL_ON;
    RCC_OscInitStruct.PLL.PLLSource = RCC_PLLSOURCE_HSE;
    RCC_OscInitStruct.PLL.PLLM = 23;
    RCC_OscInitStruct.PLL.PLLN = 354;
    RCC_OscInitStruct.PLL.PLLP = RCC_PLLP_DIV4;
    RCC_OscInitStruct.PLL.PLLQ = 8;
    if (HAL_RCC_OscConfig(&RCC_OscInitStruct) != HAL_OK) {
        Error_Handler();
    }

    /** Initializes the CPU, AHB and APB buses clocks
     */
    RCC_ClkInitStruct.ClockType =
        RCC_CLOCKTYPE_HCLK | RCC_CLOCKTYPE_SYSCLK | RCC_CLOCKTYPE_PCLK1 | RCC_CLOCKTYPE_PCLK2;
    RCC_ClkInitStruct.SYSCLKSource = RCC_SYSCLKSOURCE_PLLCLK;
    RCC_ClkInitStruct.AHBCLKDivider = RCC_SYSCLK_DIV1;
    RCC_ClkInitStruct.APB1CLKDivider = RCC_HCLK_DIV2;
    RCC_ClkInitStruct.APB2CLKDivider = RCC_HCLK_DIV1;

    if (HAL_RCC_ClockConfig(&RCC_ClkInitStruct, FLASH_LATENCY_3) != HAL_OK) {
        Error_Handler();
    }

    /** Enables the Clock Security System
     */
    HAL_RCC_EnableCSS();
}

/**
 * @brief NVIC Configuration.
 * @retval None
 */
static void MX_NVIC_Init(void) {
    /* OTG_FS_IRQn interrupt configuration */
    HAL_NVIC_SetPriority(OTG_FS_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(OTG_FS_IRQn);
    /* TIM4_IRQn interrupt configuration */
    HAL_NVIC_SetPriority(TIM4_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(TIM4_IRQn);

    HAL_NVIC_SetPriority(TIM2_IRQn, 0, 0);
    HAL_NVIC_EnableIRQ(TIM2_IRQn);
}

/**
 * @brief SPI1 Initialization Function
 * @param None
 * @retval None
 */
static void MX_SPI1_Init(void) {
    /* SPI1 parameter configuration*/
    hspi1.Instance = SPI1;
    hspi1.Init.Mode = SPI_MODE_MASTER;
    hspi1.Init.Direction = SPI_DIRECTION_2LINES_RXONLY;
    hspi1.Init.DataSize = SPI_DATASIZE_8BIT;
    hspi1.Init.CLKPolarity = SPI_POLARITY_LOW;
    hspi1.Init.CLKPhase = SPI_PHASE_1EDGE;
    hspi1.Init.NSS = SPI_NSS_SOFT;
    hspi1.Init.BaudRatePrescaler = SPI_BAUDRATEPRESCALER_32;
    hspi1.Init.FirstBit = SPI_FIRSTBIT_MSB;
    hspi1.Init.TIMode = SPI_TIMODE_DISABLE;
    hspi1.Init.CRCCalculation = SPI_CRCCALCULATION_DISABLE;
    hspi1.Init.CRCPolynomial = 10;
    if (HAL_SPI_Init(&hspi1) != HAL_OK) {
        Error_Handler();
    }
}

/**
 * @brief TIM4 Initialization Function
 * @param None
 * @retval None
 */
static void MX_TIM4_Init(void) {

    TIM_MasterConfigTypeDef sMasterConfig = {0};
    TIM_IC_InitTypeDef sConfigIC = {0};

    htim4.Instance = TIM4;
    htim4.Init.Prescaler = 0;
    htim4.Init.CounterMode = TIM_COUNTERMODE_UP;
    htim4.Init.Period = 65535;
    htim4.Init.ClockDivision = TIM_CLOCKDIVISION_DIV1;
    htim4.Init.AutoReloadPreload = TIM_AUTORELOAD_PRELOAD_DISABLE;
    if (HAL_TIM_IC_Init(&htim4) != HAL_OK) {
        Error_Handler();
    }
    sMasterConfig.MasterOutputTrigger = TIM_TRGO_RESET;
    sMasterConfig.MasterSlaveMode = TIM_MASTERSLAVEMODE_DISABLE;
    if (HAL_TIMEx_MasterConfigSynchronization(&htim4, &sMasterConfig) != HAL_OK) {
        Error_Handler();
    }
    sConfigIC.ICPolarity = TIM_INPUTCHANNELPOLARITY_RISING;
    sConfigIC.ICSelection = TIM_ICSELECTION_DIRECTTI;
    sConfigIC.ICPrescaler = TIM_ICPSC_DIV1;
    sConfigIC.ICFilter = 0xA;
    if (HAL_TIM_IC_ConfigChannel(&htim4, &sConfigIC, TIM_CHANNEL_4) != HAL_OK) {
        Error_Handler();
    }

    /* USER CODE END TIM4_Init 2 */
}

/**
 * @brief GPIO Initialization Function
 * @param None
 * @retval None
 */
static void MX_GPIO_Init(void) {
    GPIO_InitTypeDef GPIO_InitStruct = {0};

    // enable clock for GPIO
    __HAL_RCC_GPIOC_CLK_ENABLE();
    __HAL_RCC_GPIOH_CLK_ENABLE();
    __HAL_RCC_GPIOA_CLK_ENABLE();
    __HAL_RCC_GPIOB_CLK_ENABLE();

    /*Configure GPIO pin Output Level */
    HAL_GPIO_WritePin(GPIOB, MAX6675_CS_Pin | LED_INDICATORB_Pin | LED_INDICATORA_Pin,
                      GPIO_PIN_RESET);

    /*Configure GPIO pins : PC13 PC14 PC15 */
    GPIO_InitStruct.Pin = GPIO_PIN_14 | GPIO_PIN_15;
    GPIO_InitStruct.Mode = GPIO_MODE_ANALOG;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    HAL_GPIO_Init(GPIOC, &GPIO_InitStruct);

    /*Configure GPIO pins : PA0 PA1 PA2 PA3 PA4 PA7 PA8 PA9 PA10 PA13 PA14 PA15 */
    GPIO_InitStruct.Pin = GPIO_PIN_2 | GPIO_PIN_3 | GPIO_PIN_4 | GPIO_PIN_7 | GPIO_PIN_8 |
                          GPIO_PIN_9 | GPIO_PIN_10 | GPIO_PIN_13 | GPIO_PIN_14 | GPIO_PIN_15;
    GPIO_InitStruct.Mode = GPIO_MODE_ANALOG;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);

    GPIO_InitStruct.Pin = LED_STM32_Pin;
    GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
    HAL_GPIO_Init(LED_STM32_Port, &GPIO_InitStruct);

    /*Configure GPIO pins : MAX6675_CS_Pin LED_INDICATORB_Pin LED_INDICATORA_Pin
     */
    GPIO_InitStruct.Pin = MAX6675_CS_Pin | LED_INDICATORB_Pin | LED_INDICATORA_Pin;
    GPIO_InitStruct.Mode = GPIO_MODE_OUTPUT_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
    HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

    /*Configure GPIO pins : PB1 PB2 PB10 PB12 PB13 PB14 PB15 PB5 PB6 PB7 PB8 */
    GPIO_InitStruct.Pin = GPIO_PIN_1 | GPIO_PIN_2 | GPIO_PIN_10 | GPIO_PIN_12 | GPIO_PIN_13 |
                          GPIO_PIN_14 | GPIO_PIN_15 | GPIO_PIN_5 | GPIO_PIN_6 | GPIO_PIN_7 |
                          GPIO_PIN_8;
    GPIO_InitStruct.Mode = GPIO_MODE_ANALOG;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    HAL_GPIO_Init(GPIOB, &GPIO_InitStruct);

    // configure GPIO PA0 & PA1 as inputs for Encoder
    GPIO_InitStruct.Pin = GPIO_PIN_0 | GPIO_PIN_1;
    GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
    GPIO_InitStruct.Pull = GPIO_NOPULL;
    GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_HIGH;
    GPIO_InitStruct.Alternate = GPIO_AF1_TIM2;
    HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
}

/**
 * @brief  This function is executed in case of error occurrence.
 * @retval None
 */
void Error_Handler(void) {
    /* USER CODE BEGIN Error_Handler_Debug */
    /* User can add his own implementation to report the HAL error return state */
    __disable_irq();
    while (1) {
        LED_BLINKS(LED_INDICATOR_Port, LED_INDICATORB_Pin, 5, 200);
        HAL_Delay(2000);
    }
    /* USER CODE END Error_Handler_Debug */
}

#ifdef USE_FULL_ASSERT
/**
 * @brief  Reports the name of the source file and the source line number
 *         where the assert_param error has occurred.
 * @param  file: pointer to the source file name
 * @param  line: assert_param error line source number
 * @retval None
 */
void assert_failed(uint8_t *file, uint32_t line) {
    /* USER CODE BEGIN 6 */ /* User can add his own implementation to report the
                               file name and line number, ex: printf("Wrong
                               parameters value: file %s on line %d\r\n", file,
                               line) */
                            /* USER CODE END 6 */
}
#endif /* USE_FULL_ASSERT */
