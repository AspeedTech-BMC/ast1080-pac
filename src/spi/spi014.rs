#[doc = "Register `SPI014` reader"]
pub type R = crate::R<Spi014Spec>;
#[doc = "Register `SPI014` writer"]
pub type W = crate::W<Spi014Spec>;
#[doc = "Field `CSET1CMDMODE` reader - CSET1_CMDMODE"]
pub type Cset1cmdmodeR = crate::FieldReader;
#[doc = "Field `CSET1CMDMODE` writer - CSET1_CMDMODE"]
pub type Cset1cmdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET1CESTOP` reader - CSET1_CESTOP"]
pub type Cset1cestopR = crate::BitReader;
#[doc = "Field `CSET1CESTOP` writer - CSET1_CESTOP"]
pub type Cset1cestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET1BEBMODE` reader - CSET1_BEBMODE"]
pub type Cset1bebmodeR = crate::FieldReader;
#[doc = "Field `CSET1BEBMODE` writer - CSET1_BEBMODE"]
pub type Cset1bebmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET1LSBFIRST` reader - CSET1_LSBFIRST"]
pub type Cset1lsbfirstR = crate::BitReader;
#[doc = "Field `CSET1LSBFIRST` writer - CSET1_LSBFIRST"]
pub type Cset1lsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET1FRDWAITLOW` reader - CSET1_FRDWAIT_LOW"]
pub type Cset1frdwaitlowR = crate::FieldReader;
#[doc = "Field `CSET1FRDWAITLOW` writer - CSET1_FRDWAIT_LOW"]
pub type Cset1frdwaitlowW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET1CLKRATELOW` reader - CSET1_CLKRATE_LOW"]
pub type Cset1clkratelowR = crate::FieldReader;
#[doc = "Field `CSET1CLKRATELOW` writer - CSET1_CLKRATE_LOW"]
pub type Cset1clkratelowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET1DISMERGE` reader - CSET1_DISMERGE"]
pub type Cset1dismergeR = crate::BitReader;
#[doc = "Field `CSET1DISMERGE` writer - CSET1_DISMERGE"]
pub type Cset1dismergeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET1FRDWAITHIGH` reader - CSET1_FRDWAIT_HIGH"]
pub type Cset1frdwaithighR = crate::BitReader;
#[doc = "Field `CSET1FRDWAITHIGH` writer - CSET1_FRDWAIT_HIGH"]
pub type Cset1frdwaithighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET1DMYCMD` reader - CSET1_DMYCMD"]
pub type Cset1dmycmdR = crate::BitReader;
#[doc = "Field `CSET1DMYCMD` writer - CSET1_DMYCMD"]
pub type Cset1dmycmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET1CMD` reader - CSET1_CMD"]
pub type Cset1cmdR = crate::FieldReader;
#[doc = "Field `CSET1CMD` writer - CSET1_CMD"]
pub type Cset1cmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET1CLKRATEHIGH` reader - CSET1_CLKRATE_HIGH"]
pub type Cset1clkratehighR = crate::FieldReader;
#[doc = "Field `CSET1CLKRATEHIGH` writer - CSET1_CLKRATE_HIGH"]
pub type Cset1clkratehighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET1IOMODE` reader - CSET1_IOMODE"]
pub type Cset1iomodeR = crate::FieldReader;
#[doc = "Field `CSET1IOMODE` writer - CSET1_IOMODE"]
pub type Cset1iomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CSET1_CMDMODE"]
    #[inline(always)]
    pub fn cset1cmdmode(&self) -> Cset1cmdmodeR {
        Cset1cmdmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CSET1_CESTOP"]
    #[inline(always)]
    pub fn cset1cestop(&self) -> Cset1cestopR {
        Cset1cestopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - CSET1_BEBMODE"]
    #[inline(always)]
    pub fn cset1bebmode(&self) -> Cset1bebmodeR {
        Cset1bebmodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSET1_LSBFIRST"]
    #[inline(always)]
    pub fn cset1lsbfirst(&self) -> Cset1lsbfirstR {
        Cset1lsbfirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CSET1_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset1frdwaitlow(&self) -> Cset1frdwaitlowR {
        Cset1frdwaitlowR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - CSET1_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset1clkratelow(&self) -> Cset1clkratelowR {
        Cset1clkratelowR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - CSET1_DISMERGE"]
    #[inline(always)]
    pub fn cset1dismerge(&self) -> Cset1dismergeR {
        Cset1dismergeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CSET1_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset1frdwaithigh(&self) -> Cset1frdwaithighR {
        Cset1frdwaithighR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CSET1_DMYCMD"]
    #[inline(always)]
    pub fn cset1dmycmd(&self) -> Cset1dmycmdR {
        Cset1dmycmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CSET1_CMD"]
    #[inline(always)]
    pub fn cset1cmd(&self) -> Cset1cmdR {
        Cset1cmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CSET1_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset1clkratehigh(&self) -> Cset1clkratehighR {
        Cset1clkratehighR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CSET1_IOMODE"]
    #[inline(always)]
    pub fn cset1iomode(&self) -> Cset1iomodeR {
        Cset1iomodeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSET1_CMDMODE"]
    #[inline(always)]
    pub fn cset1cmdmode(&mut self) -> Cset1cmdmodeW<Spi014Spec> {
        Cset1cmdmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - CSET1_CESTOP"]
    #[inline(always)]
    pub fn cset1cestop(&mut self) -> Cset1cestopW<Spi014Spec> {
        Cset1cestopW::new(self, 2)
    }
    #[doc = "Bits 3:4 - CSET1_BEBMODE"]
    #[inline(always)]
    pub fn cset1bebmode(&mut self) -> Cset1bebmodeW<Spi014Spec> {
        Cset1bebmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - CSET1_LSBFIRST"]
    #[inline(always)]
    pub fn cset1lsbfirst(&mut self) -> Cset1lsbfirstW<Spi014Spec> {
        Cset1lsbfirstW::new(self, 5)
    }
    #[doc = "Bits 6:7 - CSET1_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset1frdwaitlow(&mut self) -> Cset1frdwaitlowW<Spi014Spec> {
        Cset1frdwaitlowW::new(self, 6)
    }
    #[doc = "Bits 8:11 - CSET1_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset1clkratelow(&mut self) -> Cset1clkratelowW<Spi014Spec> {
        Cset1clkratelowW::new(self, 8)
    }
    #[doc = "Bit 12 - CSET1_DISMERGE"]
    #[inline(always)]
    pub fn cset1dismerge(&mut self) -> Cset1dismergeW<Spi014Spec> {
        Cset1dismergeW::new(self, 12)
    }
    #[doc = "Bit 14 - CSET1_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset1frdwaithigh(&mut self) -> Cset1frdwaithighW<Spi014Spec> {
        Cset1frdwaithighW::new(self, 14)
    }
    #[doc = "Bit 15 - CSET1_DMYCMD"]
    #[inline(always)]
    pub fn cset1dmycmd(&mut self) -> Cset1dmycmdW<Spi014Spec> {
        Cset1dmycmdW::new(self, 15)
    }
    #[doc = "Bits 16:23 - CSET1_CMD"]
    #[inline(always)]
    pub fn cset1cmd(&mut self) -> Cset1cmdW<Spi014Spec> {
        Cset1cmdW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSET1_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset1clkratehigh(&mut self) -> Cset1clkratehighW<Spi014Spec> {
        Cset1clkratehighW::new(self, 24)
    }
    #[doc = "Bits 28:31 - CSET1_IOMODE"]
    #[inline(always)]
    pub fn cset1iomode(&mut self) -> Cset1iomodeW<Spi014Spec> {
        Cset1iomodeW::new(self, 28)
    }
}
#[doc = "CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi014Spec;
impl crate::RegisterSpec for Spi014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi014::R`](R) reader structure"]
impl crate::Readable for Spi014Spec {}
#[doc = "`write(|w| ..)` method takes [`spi014::W`](W) writer structure"]
impl crate::Writable for Spi014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI014 to value 0x0400"]
impl crate::Resettable for Spi014Spec {
    const RESET_VALUE: u32 = 0x0400;
}
