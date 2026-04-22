#[doc = "Register `SPI018` reader"]
pub type R = crate::R<Spi018Spec>;
#[doc = "Register `SPI018` writer"]
pub type W = crate::W<Spi018Spec>;
#[doc = "Field `CSET2CMDMODE` reader - CSET2_CMDMODE"]
pub type Cset2cmdmodeR = crate::FieldReader;
#[doc = "Field `CSET2CMDMODE` writer - CSET2_CMDMODE"]
pub type Cset2cmdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET2CESTOP` reader - CSET2_CESTOP"]
pub type Cset2cestopR = crate::BitReader;
#[doc = "Field `CSET2CESTOP` writer - CSET2_CESTOP"]
pub type Cset2cestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET2BEBMODE` reader - CSET2_BEBMODE"]
pub type Cset2bebmodeR = crate::FieldReader;
#[doc = "Field `CSET2BEBMODE` writer - CSET2_BEBMODE"]
pub type Cset2bebmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET2LSBFIRST` reader - CSET2_LSBFIRST"]
pub type Cset2lsbfirstR = crate::BitReader;
#[doc = "Field `CSET2LSBFIRST` writer - CSET2_LSBFIRST"]
pub type Cset2lsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET2FRDWAITLOW` reader - CSET2_FRDWAIT_LOW"]
pub type Cset2frdwaitlowR = crate::FieldReader;
#[doc = "Field `CSET2FRDWAITLOW` writer - CSET2_FRDWAIT_LOW"]
pub type Cset2frdwaitlowW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET2CLKRATELOW` reader - CSET2_CLKRATE_LOW"]
pub type Cset2clkratelowR = crate::FieldReader;
#[doc = "Field `CSET2CLKRATELOW` writer - CSET2_CLKRATE_LOW"]
pub type Cset2clkratelowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET2DISMERGE` reader - CSET2_DISMERGE"]
pub type Cset2dismergeR = crate::BitReader;
#[doc = "Field `CSET2DISMERGE` writer - CSET2_DISMERGE"]
pub type Cset2dismergeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET2FRDWAITHIGH` reader - CSET2_FRDWAIT_HIGH"]
pub type Cset2frdwaithighR = crate::BitReader;
#[doc = "Field `CSET2FRDWAITHIGH` writer - CSET2_FRDWAIT_HIGH"]
pub type Cset2frdwaithighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET2DMYCMD` reader - CSET2_DMYCMD"]
pub type Cset2dmycmdR = crate::BitReader;
#[doc = "Field `CSET2DMYCMD` writer - CSET2_DMYCMD"]
pub type Cset2dmycmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET2CMD` reader - CSET2_CMD"]
pub type Cset2cmdR = crate::FieldReader;
#[doc = "Field `CSET2CMD` writer - CSET2_CMD"]
pub type Cset2cmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET2CLKRATEHIGH` reader - CSET2_CLKRATE_HIGH"]
pub type Cset2clkratehighR = crate::FieldReader;
#[doc = "Field `CSET2CLKRATEHIGH` writer - CSET2_CLKRATE_HIGH"]
pub type Cset2clkratehighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET2IOMODE` reader - CSET2_IOMODE"]
pub type Cset2iomodeR = crate::FieldReader;
#[doc = "Field `CSET2IOMODE` writer - CSET2_IOMODE"]
pub type Cset2iomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CSET2_CMDMODE"]
    #[inline(always)]
    pub fn cset2cmdmode(&self) -> Cset2cmdmodeR {
        Cset2cmdmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CSET2_CESTOP"]
    #[inline(always)]
    pub fn cset2cestop(&self) -> Cset2cestopR {
        Cset2cestopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - CSET2_BEBMODE"]
    #[inline(always)]
    pub fn cset2bebmode(&self) -> Cset2bebmodeR {
        Cset2bebmodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSET2_LSBFIRST"]
    #[inline(always)]
    pub fn cset2lsbfirst(&self) -> Cset2lsbfirstR {
        Cset2lsbfirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CSET2_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset2frdwaitlow(&self) -> Cset2frdwaitlowR {
        Cset2frdwaitlowR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - CSET2_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset2clkratelow(&self) -> Cset2clkratelowR {
        Cset2clkratelowR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - CSET2_DISMERGE"]
    #[inline(always)]
    pub fn cset2dismerge(&self) -> Cset2dismergeR {
        Cset2dismergeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CSET2_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset2frdwaithigh(&self) -> Cset2frdwaithighR {
        Cset2frdwaithighR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CSET2_DMYCMD"]
    #[inline(always)]
    pub fn cset2dmycmd(&self) -> Cset2dmycmdR {
        Cset2dmycmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CSET2_CMD"]
    #[inline(always)]
    pub fn cset2cmd(&self) -> Cset2cmdR {
        Cset2cmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CSET2_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset2clkratehigh(&self) -> Cset2clkratehighR {
        Cset2clkratehighR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CSET2_IOMODE"]
    #[inline(always)]
    pub fn cset2iomode(&self) -> Cset2iomodeR {
        Cset2iomodeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSET2_CMDMODE"]
    #[inline(always)]
    pub fn cset2cmdmode(&mut self) -> Cset2cmdmodeW<Spi018Spec> {
        Cset2cmdmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - CSET2_CESTOP"]
    #[inline(always)]
    pub fn cset2cestop(&mut self) -> Cset2cestopW<Spi018Spec> {
        Cset2cestopW::new(self, 2)
    }
    #[doc = "Bits 3:4 - CSET2_BEBMODE"]
    #[inline(always)]
    pub fn cset2bebmode(&mut self) -> Cset2bebmodeW<Spi018Spec> {
        Cset2bebmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - CSET2_LSBFIRST"]
    #[inline(always)]
    pub fn cset2lsbfirst(&mut self) -> Cset2lsbfirstW<Spi018Spec> {
        Cset2lsbfirstW::new(self, 5)
    }
    #[doc = "Bits 6:7 - CSET2_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset2frdwaitlow(&mut self) -> Cset2frdwaitlowW<Spi018Spec> {
        Cset2frdwaitlowW::new(self, 6)
    }
    #[doc = "Bits 8:11 - CSET2_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset2clkratelow(&mut self) -> Cset2clkratelowW<Spi018Spec> {
        Cset2clkratelowW::new(self, 8)
    }
    #[doc = "Bit 12 - CSET2_DISMERGE"]
    #[inline(always)]
    pub fn cset2dismerge(&mut self) -> Cset2dismergeW<Spi018Spec> {
        Cset2dismergeW::new(self, 12)
    }
    #[doc = "Bit 14 - CSET2_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset2frdwaithigh(&mut self) -> Cset2frdwaithighW<Spi018Spec> {
        Cset2frdwaithighW::new(self, 14)
    }
    #[doc = "Bit 15 - CSET2_DMYCMD"]
    #[inline(always)]
    pub fn cset2dmycmd(&mut self) -> Cset2dmycmdW<Spi018Spec> {
        Cset2dmycmdW::new(self, 15)
    }
    #[doc = "Bits 16:23 - CSET2_CMD"]
    #[inline(always)]
    pub fn cset2cmd(&mut self) -> Cset2cmdW<Spi018Spec> {
        Cset2cmdW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSET2_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset2clkratehigh(&mut self) -> Cset2clkratehighW<Spi018Spec> {
        Cset2clkratehighW::new(self, 24)
    }
    #[doc = "Bits 28:31 - CSET2_IOMODE"]
    #[inline(always)]
    pub fn cset2iomode(&mut self) -> Cset2iomodeW<Spi018Spec> {
        Cset2iomodeW::new(self, 28)
    }
}
#[doc = "CE2 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi018Spec;
impl crate::RegisterSpec for Spi018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi018::R`](R) reader structure"]
impl crate::Readable for Spi018Spec {}
#[doc = "`write(|w| ..)` method takes [`spi018::W`](W) writer structure"]
impl crate::Writable for Spi018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI018 to value 0x0400"]
impl crate::Resettable for Spi018Spec {
    const RESET_VALUE: u32 = 0x0400;
}
