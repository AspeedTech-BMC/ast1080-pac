#[doc = "Register `SCU390` reader"]
pub type R = crate::R<Scu390Spec>;
#[doc = "Register `SCU390` writer"]
pub type W = crate::W<Scu390Spec>;
#[doc = "Field `SCUMACTXCLKOUTPUTDELAY` reader - SCU_MAC_TXCLK_OUTPUT_DELAY"]
pub type ScumactxclkoutputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACTXCLKOUTPUTDELAY` writer - SCU_MAC_TXCLK_OUTPUT_DELAY"]
pub type ScumactxclkoutputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLKINPUTDELAY` reader - SCU_MAC_RXCLK_INPUT_DELAY"]
pub type ScumacrxclkinputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACRXCLKINPUTDELAY` writer - SCU_MAC_RXCLK_INPUT_DELAY"]
pub type ScumacrxclkinputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLKINVERSE` reader - SCU_MAC_RXCLK_INVERSE"]
pub type ScumacrxclkinverseR = crate::BitReader;
#[doc = "Field `SCUMACRXCLKINVERSE` writer - SCU_MAC_RXCLK_INVERSE"]
pub type ScumacrxclkinverseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUMACRMIIFALLING` reader - SCU_MAC_RMII_FALLING"]
pub type ScumacrmiifallingR = crate::BitReader;
#[doc = "Field `SCUMACRMIIFALLING` writer - SCU_MAC_RMII_FALLING"]
pub type ScumacrmiifallingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUMACRMIIRCLKOE` reader - SCU_MAC_RMII_RCLK_OE"]
pub type ScumacrmiirclkoeR = crate::BitReader;
#[doc = "Field `SCUMACRMIIRCLKOE` writer - SCU_MAC_RMII_RCLK_OE"]
pub type ScumacrmiirclkoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclkoutputdelay(&self) -> ScumactxclkoutputdelayR {
        ScumactxclkoutputdelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclkinputdelay(&self) -> ScumacrxclkinputdelayR {
        ScumacrxclkinputdelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclkinverse(&self) -> ScumacrxclkinverseR {
        ScumacrxclkinverseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCU_MAC_RMII_FALLING"]
    #[inline(always)]
    pub fn scumacrmiifalling(&self) -> ScumacrmiifallingR {
        ScumacrmiifallingR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - SCU_MAC_RMII_RCLK_OE"]
    #[inline(always)]
    pub fn scumacrmiirclkoe(&self) -> ScumacrmiirclkoeR {
        ScumacrmiirclkoeR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclkoutputdelay(&mut self) -> ScumactxclkoutputdelayW<Scu390Spec> {
        ScumactxclkoutputdelayW::new(self, 0)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclkinputdelay(&mut self) -> ScumacrxclkinputdelayW<Scu390Spec> {
        ScumacrxclkinputdelayW::new(self, 12)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclkinverse(&mut self) -> ScumacrxclkinverseW<Scu390Spec> {
        ScumacrxclkinverseW::new(self, 24)
    }
    #[doc = "Bit 26 - SCU_MAC_RMII_FALLING"]
    #[inline(always)]
    pub fn scumacrmiifalling(&mut self) -> ScumacrmiifallingW<Scu390Spec> {
        ScumacrmiifallingW::new(self, 26)
    }
    #[doc = "Bit 29 - SCU_MAC_RMII_RCLK_OE"]
    #[inline(always)]
    pub fn scumacrmiirclkoe(&mut self) -> ScumacrmiirclkoeW<Scu390Spec> {
        ScumacrmiirclkoeW::new(self, 29)
    }
}
#[doc = "MAC0/1 Interface Clock Delay Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu390::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu390::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu390Spec;
impl crate::RegisterSpec for Scu390Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu390::R`](R) reader structure"]
impl crate::Readable for Scu390Spec {}
#[doc = "`write(|w| ..)` method takes [`scu390::W`](W) writer structure"]
impl crate::Writable for Scu390Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU390 to value 0"]
impl crate::Resettable for Scu390Spec {}
