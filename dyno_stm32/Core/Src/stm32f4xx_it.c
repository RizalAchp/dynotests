/* USER CODE BEGIN Header */
/**
 ******************************************************************************
 * @file    stm32f4xx_it.c
 * @brief   Interrupt Service Routines.
 ******************************************************************************
 * @attention
 *
 * Copyright (c) 2023 STMicroelectronics.
 * All rights reserved.
 *
 * This software is licensed under terms that can be found in the LICENSE file
 * in the root directory of this software component.
 * If no LICENSE file comes with this software, it is provided AS-IS.
 *
 ******************************************************************************
 */
/* USER CODE END Header */

/* Includes ------------------------------------------------------------------*/
#include "stm32f4xx_it.h"
#include "main.h"
#include <stdint.h>
/* External variables --------------------------------------------------------*/
extern PCD_HandleTypeDef hpcd_USB_OTG_FS;
extern TIM_HandleTypeDef htim4;
extern __IO uint32_t v_pulse_rpm;
#ifdef WITH_PHASE_Z
extern __IO uint32_t v_pulse_phase_z;
#endif

/******************************************************************************/
/*           Cortex-M4 Processor Interruption and Exception Handlers          */
/******************************************************************************/
/**
 * @brief This function handles Non maskable interrupt.
 */
void NMI_Handler(void) {
    HAL_RCC_NMI_IRQHandler();
    /* USER CODE BEGIN NonMaskableInt_IRQn 1 */
    while (1) {
    }
}

/**
 * @brief This function handles Hard fault interrupt.
 */
void HardFault_Handler(void) {
    while (1) {
    }
}

/**
 * @brief This function handles Memory management fault.
 */
void MemManage_Handler(void) {
    while (1) {
        LED_BLINKS(LED_STM32_Port, LED_STM32_Pin, 2, 500);
        HAL_Delay(2000);
    }
}

/**
 * @brief This function handles Pre-fetch fault, memory access fault.
 */
void BusFault_Handler(void) {
    while (1) {
        LED_BLINKS(LED_STM32_Port, LED_STM32_Pin, 4, 500);
        HAL_Delay(2000);
    }
}

/**
 * @brief This function handles Undefined instruction or illegal state.
 */
void UsageFault_Handler(void) {
    while (1) {
        LED_BLINKS(LED_STM32_Port, LED_STM32_Pin, 6, 500);
        HAL_Delay(2000);
    }
}

/**
 * @brief This function handles System service call via SWI instruction.
 */
void SVC_Handler(void) {}

/**
 * @brief This function handles Debug monitor.
 */
void DebugMon_Handler(void) {}

/**
 * @brief This function handles Pendable request for system service.
 */
void PendSV_Handler(void) {}

/**
 * @brief This function handles System tick timer.
 */
void SysTick_Handler(void) { HAL_IncTick(); }

/******************************************************************************/
/* STM32F4xx Peripheral Interrupt Handlers                                    */
/* Add here the Interrupt Handlers for the used peripherals.                  */
/* For the available peripheral interrupt handler names,                      */
/* please refer to the startup file (startup_stm32f4xx.s).                    */
/******************************************************************************/

/**
 * @brief This function handles TIM4 global interrupt.
 */
void TIM4_IRQHandler(void) {
    if (__HAL_TIM_GET_FLAG(&htim4, TIM_FLAG_CC4) != RESET) {
        if (__HAL_TIM_GET_IT_SOURCE(&htim4, TIM_IT_CC4) != RESET) {
            // Clear the interrupt flag for channel 4
            __HAL_TIM_CLEAR_IT(&htim4, TIM_IT_CC4);
            // Channel 4 is pulsing
            v_pulse_rpm += 1;
        }
    }
}

#ifdef WITH_PHASE_Z
void TIM5_IRQHandler(void) {
    if (__HAL_TIM_GET_FLAG(&htim5, TIM_FLAG_CC3) != RESET) {
        if (__HAL_TIM_GET_IT_SOURCE(&htim5, TIM_IT_CC3) != RESET) {
            // Clear the interrupt flag for channel 4
            __HAL_TIM_CLEAR_IT(&htim5, TIM_IT_CC3);
            // Channel 4 is pulsing
            v_pulse_phase_z += 1;
        }
    }
}
#endif

/**
 * @brief This function handles USB On The Go FS global interrupt.
 */
void OTG_FS_IRQHandler(void) { HAL_PCD_IRQHandler(&hpcd_USB_OTG_FS); }
