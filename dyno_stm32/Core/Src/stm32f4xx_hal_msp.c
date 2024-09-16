/**
 ******************************************************************************
 * @file         stm32f4xx_hal_msp.c
 * @brief        This file provides code for the MSP Initialization
 *               and de-Initialization codes.
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

/* Includes ------------------------------------------------------------------*/
#include "main.h"
/**
 * Initializes the Global MSP.
 */
void HAL_MspInit(void) {
    __HAL_RCC_SYSCFG_CLK_ENABLE();
    __HAL_RCC_PWR_CLK_ENABLE();
    /* System interrupt init*/
}

/**
 * @brief SPI MSP Initialization
 * This function configures the hardware resources used in this example
 * @param hspi: SPI handle pointer
 * @retval None
 */
void HAL_SPI_MspInit(SPI_HandleTypeDef *hspi) {
    GPIO_InitTypeDef GPIO_InitStruct = {0};
    if (hspi->Instance == SPI1) {
        __HAL_RCC_SPI1_CLK_ENABLE();
        __HAL_RCC_GPIOA_CLK_ENABLE();
        /**SPI1 GPIO Configuration
        PA5     ------> SPI1_SCK
        PA6     ------> SPI1_MISO
        */
        GPIO_InitStruct.Pin = GPIO_PIN_5 | GPIO_PIN_6;
        GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
        GPIO_InitStruct.Pull = GPIO_NOPULL;
        GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_VERY_HIGH;
        GPIO_InitStruct.Alternate = GPIO_AF5_SPI1;
        HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
    }
}

/**
 * @brief SPI MSP De-Initialization
 * This function freeze the hardware resources used in this example
 * @param hspi: SPI handle pointer
 * @retval None
 */
void HAL_SPI_MspDeInit(SPI_HandleTypeDef *hspi) {
    if (hspi->Instance == SPI1) {
        __HAL_RCC_SPI1_CLK_DISABLE();

        /**SPI1 GPIO Configuration
        PA5     ------> SPI1_SCK
        PA6     ------> SPI1_MISO
        */
        HAL_GPIO_DeInit(GPIOA, GPIO_PIN_5 | GPIO_PIN_6);
    }
}

/**
 * @brief TIM_IC MSP Initialization
 * This function configures the hardware resources used in this example
 * @param htim_ic: TIM_IC handle pointer
 * @retval None
 */
void HAL_TIM_IC_MspInit(TIM_HandleTypeDef *htim_ic) {
    GPIO_InitTypeDef GPIO_InitStruct = {0};
    if (htim_ic->Instance == TIM4) {
        /* Peripheral clock enable */
        __HAL_RCC_TIM4_CLK_ENABLE();
        __HAL_RCC_GPIOB_CLK_ENABLE();
        /**TIM4 GPIO Configuration
        PB9     ------> TIM4_CH4
        */
        GPIO_InitStruct.Pin = RPM_INPUT_CAPTURE_Pin;
        GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
        GPIO_InitStruct.Pull = GPIO_PULLDOWN;
        GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
        GPIO_InitStruct.Alternate = GPIO_AF2_TIM4;
        HAL_GPIO_Init(RPM_INPUT_CAPTURE_GPIO_Port, &GPIO_InitStruct);
    }
#ifdef WITH_PHASE_Z
    else if (htim_ic->Instance == TIM5) {
        /* Peripheral clock enable */
        __HAL_RCC_TIM5_CLK_ENABLE();
        __HAL_RCC_GPIOA_CLK_ENABLE();
        /**TIM4 GPIO Configuration
        PB9     ------> TIM4_CH4
        */
        GPIO_InitStruct.Pin = PHASE_Z_Pin;
        GPIO_InitStruct.Mode = GPIO_MODE_AF_PP;
        GPIO_InitStruct.Pull = GPIO_PULLDOWN;
        GPIO_InitStruct.Speed = GPIO_SPEED_FREQ_LOW;
        GPIO_InitStruct.Alternate = GPIO_AF2_TIM4;
        HAL_GPIO_Init(GPIOA, &GPIO_InitStruct);
    }
#endif
}

/**
 * @brief TIM_IC MSP De-Initialization
 * This function freeze the hardware resources used in this example
 * @param htim_ic: TIM_IC handle pointer
 * @retval None
 */
void HAL_TIM_IC_MspDeInit(TIM_HandleTypeDef *htim_ic) {
    if (htim_ic->Instance == TIM4) {
        /* Peripheral clock disable */
        __HAL_RCC_TIM4_CLK_DISABLE();

        /**TIM4 GPIO Configuration
        PB9     ------> TIM4_CH4
        */
        HAL_GPIO_DeInit(RPM_INPUT_CAPTURE_GPIO_Port, RPM_INPUT_CAPTURE_Pin);
        HAL_NVIC_DisableIRQ(TIM4_IRQn);
    }
#ifdef WITH_PHASE_Z
    else if (htim_ic->Instance == TIM5) {
        /* Peripheral clock disable */
        __HAL_RCC_TIM4_CLK_DISABLE();

        /**TIM4 GPIO Configuration
        PB9     ------> TIM4_CH4
        */
        HAL_GPIO_DeInit(GPIOA, PHASE_Z_Pin);
        HAL_NVIC_DisableIRQ(TIM5_IRQn);
    }
#endif
}
