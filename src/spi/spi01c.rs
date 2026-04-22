#[doc = "Register `SPI01C` reader"]
pub type R = crate::R<Spi01cSpec>;
#[doc = "Register `SPI01C` writer"]
pub type W = crate::W<Spi01cSpec>;
#[doc = "Field `CSET3CMDMODE` reader - CSET3_CMDMODE"]
pub type Cset3cmdmodeR = crate::FieldReader;
#[doc = "Field `CSET3CMDMODE` writer - CSET3_CMDMODE"]
pub type Cset3cmdmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET3CESTOP` reader - CSET3_CESTOP"]
pub type Cset3cestopR = crate::BitReader;
#[doc = "Field `CSET3CESTOP` writer - CSET3_CESTOP"]
pub type Cset3cestopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET3BEBMODE` reader - CSET3_BEBMODE"]
pub type Cset3bebmodeR = crate::FieldReader;
#[doc = "Field `CSET3BEBMODE` writer - CSET3_BEBMODE"]
pub type Cset3bebmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET3LSBFIRST` reader - CSET3_LSBFIRST"]
pub type Cset3lsbfirstR = crate::BitReader;
#[doc = "Field `CSET3LSBFIRST` writer - CSET3_LSBFIRST"]
pub type Cset3lsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET3FRDWAITLOW` reader - CSET3_FRDWAIT_LOW"]
pub type Cset3frdwaitlowR = crate::FieldReader;
#[doc = "Field `CSET3FRDWAITLOW` writer - CSET3_FRDWAIT_LOW"]
pub type Cset3frdwaitlowW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CSET3CLKRATELOW` reader - CSET3_CLKRATE_LOW"]
pub type Cset3clkratelowR = crate::FieldReader;
#[doc = "Field `CSET3CLKRATELOW` writer - CSET3_CLKRATE_LOW"]
pub type Cset3clkratelowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET3DISMERGE` reader - CSET3_DISMERGE"]
pub type Cset3dismergeR = crate::BitReader;
#[doc = "Field `CSET3DISMERGE` writer - CSET3_DISMERGE"]
pub type Cset3dismergeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET3FRDWAITHIGH` reader - CSET3_FRDWAIT_HIGH"]
pub type Cset3frdwaithighR = crate::BitReader;
#[doc = "Field `CSET3FRDWAITHIGH` writer - CSET3_FRDWAIT_HIGH"]
pub type Cset3frdwaithighW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET3DMYCMD` reader - CSET3_DMYCMD"]
pub type Cset3dmycmdR = crate::BitReader;
#[doc = "Field `CSET3DMYCMD` writer - CSET3_DMYCMD"]
pub type Cset3dmycmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSET3CMD` reader - CSET3_CMD"]
pub type Cset3cmdR = crate::FieldReader;
#[doc = "Field `CSET3CMD` writer - CSET3_CMD"]
pub type Cset3cmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CSET3CLKRATEHIGH` reader - CSET3_CLKRATE_HIGH"]
pub type Cset3clkratehighR = crate::FieldReader;
#[doc = "Field `CSET3CLKRATEHIGH` writer - CSET3_CLKRATE_HIGH"]
pub type Cset3clkratehighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CSET3IOMODE` reader - CSET3_IOMODE"]
pub type Cset3iomodeR = crate::FieldReader;
#[doc = "Field `CSET3IOMODE` writer - CSET3_IOMODE"]
pub type Cset3iomodeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - CSET3_CMDMODE"]
    #[inline(always)]
    pub fn cset3cmdmode(&self) -> Cset3cmdmodeR {
        Cset3cmdmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - CSET3_CESTOP"]
    #[inline(always)]
    pub fn cset3cestop(&self) -> Cset3cestopR {
        Cset3cestopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - CSET3_BEBMODE"]
    #[inline(always)]
    pub fn cset3bebmode(&self) -> Cset3bebmodeR {
        Cset3bebmodeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - CSET3_LSBFIRST"]
    #[inline(always)]
    pub fn cset3lsbfirst(&self) -> Cset3lsbfirstR {
        Cset3lsbfirstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - CSET3_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset3frdwaitlow(&self) -> Cset3frdwaitlowR {
        Cset3frdwaitlowR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - CSET3_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset3clkratelow(&self) -> Cset3clkratelowR {
        Cset3clkratelowR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - CSET3_DISMERGE"]
    #[inline(always)]
    pub fn cset3dismerge(&self) -> Cset3dismergeR {
        Cset3dismergeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - CSET3_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset3frdwaithigh(&self) -> Cset3frdwaithighR {
        Cset3frdwaithighR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CSET3_DMYCMD"]
    #[inline(always)]
    pub fn cset3dmycmd(&self) -> Cset3dmycmdR {
        Cset3dmycmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - CSET3_CMD"]
    #[inline(always)]
    pub fn cset3cmd(&self) -> Cset3cmdR {
        Cset3cmdR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - CSET3_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset3clkratehigh(&self) -> Cset3clkratehighR {
        Cset3clkratehighR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - CSET3_IOMODE"]
    #[inline(always)]
    pub fn cset3iomode(&self) -> Cset3iomodeR {
        Cset3iomodeR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CSET3_CMDMODE"]
    #[inline(always)]
    pub fn cset3cmdmode(&mut self) -> Cset3cmdmodeW<Spi01cSpec> {
        Cset3cmdmodeW::new(self, 0)
    }
    #[doc = "Bit 2 - CSET3_CESTOP"]
    #[inline(always)]
    pub fn cset3cestop(&mut self) -> Cset3cestopW<Spi01cSpec> {
        Cset3cestopW::new(self, 2)
    }
    #[doc = "Bits 3:4 - CSET3_BEBMODE"]
    #[inline(always)]
    pub fn cset3bebmode(&mut self) -> Cset3bebmodeW<Spi01cSpec> {
        Cset3bebmodeW::new(self, 3)
    }
    #[doc = "Bit 5 - CSET3_LSBFIRST"]
    #[inline(always)]
    pub fn cset3lsbfirst(&mut self) -> Cset3lsbfirstW<Spi01cSpec> {
        Cset3lsbfirstW::new(self, 5)
    }
    #[doc = "Bits 6:7 - CSET3_FRDWAIT_LOW"]
    #[inline(always)]
    pub fn cset3frdwaitlow(&mut self) -> Cset3frdwaitlowW<Spi01cSpec> {
        Cset3frdwaitlowW::new(self, 6)
    }
    #[doc = "Bits 8:11 - CSET3_CLKRATE_LOW"]
    #[inline(always)]
    pub fn cset3clkratelow(&mut self) -> Cset3clkratelowW<Spi01cSpec> {
        Cset3clkratelowW::new(self, 8)
    }
    #[doc = "Bit 12 - CSET3_DISMERGE"]
    #[inline(always)]
    pub fn cset3dismerge(&mut self) -> Cset3dismergeW<Spi01cSpec> {
        Cset3dismergeW::new(self, 12)
    }
    #[doc = "Bit 14 - CSET3_FRDWAIT_HIGH"]
    #[inline(always)]
    pub fn cset3frdwaithigh(&mut self) -> Cset3frdwaithighW<Spi01cSpec> {
        Cset3frdwaithighW::new(self, 14)
    }
    #[doc = "Bit 15 - CSET3_DMYCMD"]
    #[inline(always)]
    pub fn cset3dmycmd(&mut self) -> Cset3dmycmdW<Spi01cSpec> {
        Cset3dmycmdW::new(self, 15)
    }
    #[doc = "Bits 16:23 - CSET3_CMD"]
    #[inline(always)]
    pub fn cset3cmd(&mut self) -> Cset3cmdW<Spi01cSpec> {
        Cset3cmdW::new(self, 16)
    }
    #[doc = "Bits 24:27 - CSET3_CLKRATE_HIGH"]
    #[inline(always)]
    pub fn cset3clkratehigh(&mut self) -> Cset3clkratehighW<Spi01cSpec> {
        Cset3clkratehighW::new(self, 24)
    }
    #[doc = "Bits 28:31 - CSET3_IOMODE"]
    #[inline(always)]
    pub fn cset3iomode(&mut self) -> Cset3iomodeW<Spi01cSpec> {
        Cset3iomodeW::new(self, 28)
    }
}
#[doc = "CE3 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi01cSpec;
impl crate::RegisterSpec for Spi01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi01c::R`](R) reader structure"]
impl crate::Readable for Spi01cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi01c::W`](W) writer structure"]
impl crate::Writable for Spi01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI01C to value 0x0400"]
impl crate::Resettable for Spi01cSpec {
    const RESET_VALUE: u32 = 0x0400;
}
