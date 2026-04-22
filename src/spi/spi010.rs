#[doc = "Register `SPI010` reader"]
pub type R = crate::R<Spi010Spec>;
#[doc = "Register `SPI010` writer"]
pub type W = crate::W<Spi010Spec>;
#[doc = "Field `CSET0CMDMODE` reader - CSET0_CMDMODE"]
pub type Cset0cmdmodeR = crate::FieldReader;
#[doc = "Field `CSET0CMDMODE` writer - CSET0_CMDMODE"]
pub type Cset0cmdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET0CESTOP` reader - CSET0_CESTOP"]
pub type Cset0cestopR = crate::BitReader;
#[doc = "Field `CSET0CESTOP` writer - CSET0_CESTOP"]
pub type Cset0cestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET0BEBMODE` reader - CSET0_BEBMODE"]
pub type Cset0bebmodeR = crate::FieldReader;
#[doc = "Field `CSET0BEBMODE` writer - CSET0_BEBMODE"]
pub type Cset0bebmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET0LSBFIRST` reader - CSET0_LSBFIRST"]
pub type Cset0lsbfirstR = crate::BitReader;
#[doc = "Field `CSET0LSBFIRST` writer - CSET0_LSBFIRST"]
pub type Cset0lsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET0FRDWAITLOW` reader - CSET0_FRDWAIT_LOW"]
pub type Cset0frdwaitlowR = crate::FieldReader;
#[doc = "Field `CSET0FRDWAITLOW` writer - CSET0_FRDWAIT_LOW"]
pub type Cset0frdwaitlowW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET0CLKRATELOW` reader - CSET0_CLKRATE_LOW"]
pub type Cset0clkratelowR = crate::FieldReader;
#[doc = "Field `CSET0CLKRATELOW` writer - CSET0_CLKRATE_LOW"]
pub type Cset0clkratelowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET0DISMERGE` reader - CSET0_DISMERGE"]
pub type Cset0dismergeR = crate::BitReader;
#[doc = "Field `CSET0DISMERGE` writer - CSET0_DISMERGE"]
pub type Cset0dismergeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET0FRDWAITHIGH` reader - CSET0_FRDWAIT_HIGH"]
pub type Cset0frdwaithighR = crate::BitReader;
#[doc = "Field `CSET0FRDWAITHIGH` writer - CSET0_FRDWAIT_HIGH"]
pub type Cset0frdwaithighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET0DMYCMD` reader - CSET0_DMYCMD"]
pub type Cset0dmycmdR = crate::BitReader;
#[doc = "Field `CSET0DMYCMD` writer - CSET0_DMYCMD"]
pub type Cset0dmycmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET0CMD` reader - CSET0_CMD"]
pub type Cset0cmdR = crate::FieldReader;
#[doc = "Field `CSET0CMD` writer - CSET0_CMD"]
pub type Cset0cmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET0CLKRATEHIGH` reader - CSET0_CLKRATE_HIGH"]
pub type Cset0clkratehighR = crate::FieldReader;
#[doc = "Field `CSET0CLKRATEHIGH` writer - CSET0_CLKRATE_HIGH"]
pub type Cset0clkratehighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET0IOMODE` reader - CSET0_IOMODE"]
pub type Cset0iomodeR = crate::FieldReader;
#[doc = "Field `CSET0IOMODE` writer - CSET0_IOMODE"]
pub type Cset0iomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CSET0_CMDMODE"]
    #[inline(always)]
    pub fn cset0cmdmode(&self) -> Cset0cmdmodeR {
        Cset0cmdmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CSET0_CESTOP"]
    #[inline(always)]
    pub fn cset0cestop(&self) -> Cset0cestopR {
        Cset0cestopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - CSET0_BEBMODE"]
    #[inline(always)]
    pub fn cset0bebmode(&self) -> Cset0bebmodeR {
        Cset0bebmodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSET0_LSBFIRST"]
    #[inline(always)]
    pub fn cset0lsbfirst(&self) -> Cset0lsbfirstR {
        Cset0lsbfirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CSET0_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset0frdwaitlow(&self) -> Cset0frdwaitlowR {
        Cset0frdwaitlowR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - CSET0_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset0clkratelow(&self) -> Cset0clkratelowR {
        Cset0clkratelowR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - CSET0_DISMERGE"]
    #[inline(always)]
    pub fn cset0dismerge(&self) -> Cset0dismergeR {
        Cset0dismergeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CSET0_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset0frdwaithigh(&self) -> Cset0frdwaithighR {
        Cset0frdwaithighR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CSET0_DMYCMD"]
    #[inline(always)]
    pub fn cset0dmycmd(&self) -> Cset0dmycmdR {
        Cset0dmycmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CSET0_CMD"]
    #[inline(always)]
    pub fn cset0cmd(&self) -> Cset0cmdR {
        Cset0cmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CSET0_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset0clkratehigh(&self) -> Cset0clkratehighR {
        Cset0clkratehighR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CSET0_IOMODE"]
    #[inline(always)]
    pub fn cset0iomode(&self) -> Cset0iomodeR {
        Cset0iomodeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSET0_CMDMODE"]
    #[inline(always)]
    pub fn cset0cmdmode(&mut self) -> Cset0cmdmodeW<Spi010Spec> {
        Cset0cmdmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - CSET0_CESTOP"]
    #[inline(always)]
    pub fn cset0cestop(&mut self) -> Cset0cestopW<Spi010Spec> {
        Cset0cestopW::new(self, 2)
    }
    #[doc = "Bits 3:4 - CSET0_BEBMODE"]
    #[inline(always)]
    pub fn cset0bebmode(&mut self) -> Cset0bebmodeW<Spi010Spec> {
        Cset0bebmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - CSET0_LSBFIRST"]
    #[inline(always)]
    pub fn cset0lsbfirst(&mut self) -> Cset0lsbfirstW<Spi010Spec> {
        Cset0lsbfirstW::new(self, 5)
    }
    #[doc = "Bits 6:7 - CSET0_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset0frdwaitlow(&mut self) -> Cset0frdwaitlowW<Spi010Spec> {
        Cset0frdwaitlowW::new(self, 6)
    }
    #[doc = "Bits 8:11 - CSET0_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset0clkratelow(&mut self) -> Cset0clkratelowW<Spi010Spec> {
        Cset0clkratelowW::new(self, 8)
    }
    #[doc = "Bit 12 - CSET0_DISMERGE"]
    #[inline(always)]
    pub fn cset0dismerge(&mut self) -> Cset0dismergeW<Spi010Spec> {
        Cset0dismergeW::new(self, 12)
    }
    #[doc = "Bit 14 - CSET0_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset0frdwaithigh(&mut self) -> Cset0frdwaithighW<Spi010Spec> {
        Cset0frdwaithighW::new(self, 14)
    }
    #[doc = "Bit 15 - CSET0_DMYCMD"]
    #[inline(always)]
    pub fn cset0dmycmd(&mut self) -> Cset0dmycmdW<Spi010Spec> {
        Cset0dmycmdW::new(self, 15)
    }
    #[doc = "Bits 16:23 - CSET0_CMD"]
    #[inline(always)]
    pub fn cset0cmd(&mut self) -> Cset0cmdW<Spi010Spec> {
        Cset0cmdW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSET0_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset0clkratehigh(&mut self) -> Cset0clkratehighW<Spi010Spec> {
        Cset0clkratehighW::new(self, 24)
    }
    #[doc = "Bits 28:31 - CSET0_IOMODE"]
    #[inline(always)]
    pub fn cset0iomode(&mut self) -> Cset0iomodeW<Spi010Spec> {
        Cset0iomodeW::new(self, 28)
    }
}
#[doc = "CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi010::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi010::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi010Spec;
impl crate::RegisterSpec for Spi010Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi010::R`](R) reader structure"]
impl crate::Readable for Spi010Spec {}
#[doc = "`write(|w| ..)` method takes [`spi010::W`](W) writer structure"]
impl crate::Writable for Spi010Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI010 to value 0x0400"]
impl crate::Resettable for Spi010Spec {
    const RESET_VALUE: u32 = 0x0400;
}
