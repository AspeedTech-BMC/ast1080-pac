#[doc = "Register `SCU284` reader"]
pub type R = crate::R<Scu284Spec>;
#[doc = "Register `SCU284` writer"]
pub type W = crate::W<Scu284Spec>;
#[doc = "Field `SCUUXCLKSEL` reader - SCU_UXCLK_SEL"]
pub type ScuuxclkselR = crate::FieldReader;
#[doc = "Field `SCUUXCLKSEL` writer - SCU_UXCLK_SEL"]
pub type ScuuxclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCUHUXCLKSEL` reader - SCU_HUXCLK_SEL"]
pub type ScuhuxclkselR = crate::FieldReader;
#[doc = "Field `SCUHUXCLKSEL` writer - SCU_HUXCLK_SEL"]
pub type ScuhuxclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUPSPCLKDIV` reader - SCU_PSPCLK_DIV"]
pub type ScupspclkdivR = crate::FieldReader;
#[doc = "Field `SCUPSPCLKDIV` writer - SCU_PSPCLK_DIV"]
pub type ScupspclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCUSSPCLKDIV` reader - SCU_SSPCLK_DIV"]
pub type ScusspclkdivR = crate::FieldReader;
#[doc = "Field `SCUSSPCLKDIV` writer - SCU_SSPCLK_DIV"]
pub type ScusspclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUCPTRACLKDIV` reader - SCU_CPTRACLK_DIV"]
pub type ScucptraclkdivR = crate::FieldReader;
#[doc = "Field `SCUCPTRACLKDIV` writer - SCU_CPTRACLK_DIV"]
pub type ScucptraclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUPECICLKSEL` reader - SCU_PECICLK_SEL"]
pub type ScupeciclkselR = crate::BitReader;
#[doc = "Field `SCUPECICLKSEL` writer - SCU_PECICLK_SEL"]
pub type ScupeciclkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCURTCDIV` reader - SCU_RTC_DIV"]
pub type ScurtcdivR = crate::BitReader;
#[doc = "Field `SCURTCDIV` writer - SCU_RTC_DIV"]
pub type ScurtcdivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUHCLKDIV` reader - SCU_HCLK_DIV"]
pub type ScuhclkdivR = crate::FieldReader;
#[doc = "Field `SCUHCLKDIV` writer - SCU_HCLK_DIV"]
pub type ScuhclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCUI3CCLKDIV` reader - SCU_I3CCLK_DIV"]
pub type Scui3cclkdivR = crate::FieldReader;
#[doc = "Field `SCUI3CCLKDIV` writer - SCU_I3CCLK_DIV"]
pub type Scui3cclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCUSPIS0CLKDIV` reader - SCU_SPIS0CLK_DIV"]
pub type Scuspis0clkdivR = crate::FieldReader;
#[doc = "Field `SCUSPIS0CLKDIV` writer - SCU_SPIS0CLK_DIV"]
pub type Scuspis0clkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCUSPIS1CLKDIV` reader - SCU_SPIS1CLK_DIV"]
pub type Scuspis1clkdivR = crate::FieldReader;
#[doc = "Field `SCUSPIS1CLKDIV` writer - SCU_SPIS1CLK_DIV"]
pub type Scuspis1clkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - SCU_UXCLK_SEL"]
    #[inline(always)]
    pub fn scuuxclksel(&self) -> ScuuxclkselR {
        ScuuxclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - SCU_HUXCLK_SEL"]
    #[inline(always)]
    pub fn scuhuxclksel(&self) -> ScuhuxclkselR {
        ScuhuxclkselR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:7 - SCU_PSPCLK_DIV"]
    #[inline(always)]
    pub fn scupspclkdiv(&self) -> ScupspclkdivR {
        ScupspclkdivR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - SCU_SSPCLK_DIV"]
    #[inline(always)]
    pub fn scusspclkdiv(&self) -> ScusspclkdivR {
        ScusspclkdivR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - SCU_CPTRACLK_DIV"]
    #[inline(always)]
    pub fn scucptraclkdiv(&self) -> ScucptraclkdivR {
        ScucptraclkdivR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_PECICLK_SEL"]
    #[inline(always)]
    pub fn scupeciclksel(&self) -> ScupeciclkselR {
        ScupeciclkselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCU_RTC_DIV"]
    #[inline(always)]
    pub fn scurtcdiv(&self) -> ScurtcdivR {
        ScurtcdivR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:22 - SCU_HCLK_DIV"]
    #[inline(always)]
    pub fn scuhclkdiv(&self) -> ScuhclkdivR {
        ScuhclkdivR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 23:25 - SCU_I3CCLK_DIV"]
    #[inline(always)]
    pub fn scui3cclkdiv(&self) -> Scui3cclkdivR {
        Scui3cclkdivR::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bits 26:28 - SCU_SPIS0CLK_DIV"]
    #[inline(always)]
    pub fn scuspis0clkdiv(&self) -> Scuspis0clkdivR {
        Scuspis0clkdivR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - SCU_SPIS1CLK_DIV"]
    #[inline(always)]
    pub fn scuspis1clkdiv(&self) -> Scuspis1clkdivR {
        Scuspis1clkdivR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SCU_UXCLK_SEL"]
    #[inline(always)]
    pub fn scuuxclksel(&mut self) -> ScuuxclkselW<Scu284Spec> {
        ScuuxclkselW::new(self, 0)
    }
    #[doc = "Bits 3:4 - SCU_HUXCLK_SEL"]
    #[inline(always)]
    pub fn scuhuxclksel(&mut self) -> ScuhuxclkselW<Scu284Spec> {
        ScuhuxclkselW::new(self, 3)
    }
    #[doc = "Bits 5:7 - SCU_PSPCLK_DIV"]
    #[inline(always)]
    pub fn scupspclkdiv(&mut self) -> ScupspclkdivW<Scu284Spec> {
        ScupspclkdivW::new(self, 5)
    }
    #[doc = "Bits 8:10 - SCU_SSPCLK_DIV"]
    #[inline(always)]
    pub fn scusspclkdiv(&mut self) -> ScusspclkdivW<Scu284Spec> {
        ScusspclkdivW::new(self, 8)
    }
    #[doc = "Bits 12:14 - SCU_CPTRACLK_DIV"]
    #[inline(always)]
    pub fn scucptraclkdiv(&mut self) -> ScucptraclkdivW<Scu284Spec> {
        ScucptraclkdivW::new(self, 12)
    }
    #[doc = "Bit 16 - SCU_PECICLK_SEL"]
    #[inline(always)]
    pub fn scupeciclksel(&mut self) -> ScupeciclkselW<Scu284Spec> {
        ScupeciclkselW::new(self, 16)
    }
    #[doc = "Bit 17 - SCU_RTC_DIV"]
    #[inline(always)]
    pub fn scurtcdiv(&mut self) -> ScurtcdivW<Scu284Spec> {
        ScurtcdivW::new(self, 17)
    }
    #[doc = "Bits 20:22 - SCU_HCLK_DIV"]
    #[inline(always)]
    pub fn scuhclkdiv(&mut self) -> ScuhclkdivW<Scu284Spec> {
        ScuhclkdivW::new(self, 20)
    }
    #[doc = "Bits 23:25 - SCU_I3CCLK_DIV"]
    #[inline(always)]
    pub fn scui3cclkdiv(&mut self) -> Scui3cclkdivW<Scu284Spec> {
        Scui3cclkdivW::new(self, 23)
    }
    #[doc = "Bits 26:28 - SCU_SPIS0CLK_DIV"]
    #[inline(always)]
    pub fn scuspis0clkdiv(&mut self) -> Scuspis0clkdivW<Scu284Spec> {
        Scuspis0clkdivW::new(self, 26)
    }
    #[doc = "Bits 29:31 - SCU_SPIS1CLK_DIV"]
    #[inline(always)]
    pub fn scuspis1clkdiv(&mut self) -> Scuspis1clkdivW<Scu284Spec> {
        Scuspis1clkdivW::new(self, 29)
    }
}
#[doc = "Clock Selection 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu284::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu284::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu284Spec;
impl crate::RegisterSpec for Scu284Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu284::R`](R) reader structure"]
impl crate::Readable for Scu284Spec {}
#[doc = "`write(|w| ..)` method takes [`scu284::W`](W) writer structure"]
impl crate::Writable for Scu284Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU284 to value 0x9230_1480"]
impl crate::Resettable for Scu284Spec {
    const RESET_VALUE: u32 = 0x9230_1480;
}
