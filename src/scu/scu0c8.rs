#[doc = "Register `SCU0C8` reader"]
pub type R = crate::R<Scu0c8Spec>;
#[doc = "Register `SCU0C8` writer"]
pub type W = crate::W<Scu0c8Spec>;
#[doc = "Field `SCUDISESPIAHB` reader - SCU_DIS_ESPI_AHB"]
pub type ScudisespiahbR = crate::BitReader;
#[doc = "Field `SCUDISESPIAHB` writer - SCU_DIS_ESPI_AHB"]
pub type ScudisespiahbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISLPCBRIDGE1` reader - SCU_DIS_LPC_BRIDGE_1"]
pub type Scudislpcbridge1R = crate::BitReader;
#[doc = "Field `SCUDISLPCBRIDGE1` writer - SCU_DIS_LPC_BRIDGE_1"]
pub type Scudislpcbridge1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDISLPCBRIDGE2` reader - SCU_DIS_LPC_BRIDGE_2"]
pub type Scudislpcbridge2R = crate::BitReader;
#[doc = "Field `SCUDISLPCBRIDGE2` writer - SCU_DIS_LPC_BRIDGE_2"]
pub type Scudislpcbridge2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUDISUARTDBG` reader - SCU_DIS_UART_DBG"]
pub type ScudisuartdbgR = crate::BitReader;
#[doc = "Field `SCUDISUARTDBG` writer - SCU_DIS_UART_DBG"]
pub type ScudisuartdbgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUDISWDTRSTFULL` reader - SCU_DIS_WDT_RSTFULL"]
pub type ScudiswdtrstfullR = crate::BitReader;
#[doc = "Field `SCUDISWDTRSTFULL` writer - SCU_DIS_WDT_RSTFULL"]
pub type ScudiswdtrstfullW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_ESPI_AHB"]
    #[inline(always)]
    pub fn scudisespiahb(&self) -> ScudisespiahbR {
        ScudisespiahbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_LPC_BRIDGE_1"]
    #[inline(always)]
    pub fn scudislpcbridge1(&self) -> Scudislpcbridge1R {
        Scudislpcbridge1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_DIS_LPC_BRIDGE_2"]
    #[inline(always)]
    pub fn scudislpcbridge2(&self) -> Scudislpcbridge2R {
        Scudislpcbridge2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - SCU_DIS_UART_DBG"]
    #[inline(always)]
    pub fn scudisuartdbg(&self) -> ScudisuartdbgR {
        ScudisuartdbgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SCU_DIS_WDT_RSTFULL"]
    #[inline(always)]
    pub fn scudiswdtrstfull(&self) -> ScudiswdtrstfullR {
        ScudiswdtrstfullR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_ESPI_AHB"]
    #[inline(always)]
    pub fn scudisespiahb(&mut self) -> ScudisespiahbW<Scu0c8Spec> {
        ScudisespiahbW::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_LPC_BRIDGE_1"]
    #[inline(always)]
    pub fn scudislpcbridge1(&mut self) -> Scudislpcbridge1W<Scu0c8Spec> {
        Scudislpcbridge1W::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_DIS_LPC_BRIDGE_2"]
    #[inline(always)]
    pub fn scudislpcbridge2(&mut self) -> Scudislpcbridge2W<Scu0c8Spec> {
        Scudislpcbridge2W::new(self, 3)
    }
    #[doc = "Bit 6 - SCU_DIS_UART_DBG"]
    #[inline(always)]
    pub fn scudisuartdbg(&mut self) -> ScudisuartdbgW<Scu0c8Spec> {
        ScudisuartdbgW::new(self, 6)
    }
    #[doc = "Bit 7 - SCU_DIS_WDT_RSTFULL"]
    #[inline(always)]
    pub fn scudiswdtrstfull(&mut self) -> ScudiswdtrstfullW<Scu0c8Spec> {
        ScudiswdtrstfullW::new(self, 7)
    }
}
#[doc = "Debug Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0c8Spec;
impl crate::RegisterSpec for Scu0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0c8::R`](R) reader structure"]
impl crate::Readable for Scu0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0c8::W`](W) writer structure"]
impl crate::Writable for Scu0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0C8 to value 0"]
impl crate::Resettable for Scu0c8Spec {}
