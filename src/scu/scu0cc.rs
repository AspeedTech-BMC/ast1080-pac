#[doc = "Register `SCU0CC` reader"]
pub type R = crate::R<Scu0ccSpec>;
#[doc = "Register `SCU0CC` writer"]
pub type W = crate::W<Scu0ccSpec>;
#[doc = "Field `SCUHBSWCNT` reader - SCU_HB_SW_CNT"]
pub type ScuhbswcntR = crate::FieldReader<u16>;
#[doc = "Field `SCUHBSWCNT` writer - SCU_HB_SW_CNT"]
pub type ScuhbswcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUHBSWEN` reader - SCU_HB_SW_EN"]
pub type ScuhbswenR = crate::BitReader;
#[doc = "Field `SCUHBSWEN` writer - SCU_HB_SW_EN"]
pub type ScuhbswenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - SCU_HB_SW_CNT"]
    #[inline(always)]
    pub fn scuhbswcnt(&self) -> ScuhbswcntR {
        ScuhbswcntR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_HB_SW_EN"]
    #[inline(always)]
    pub fn scuhbswen(&self) -> ScuhbswenR {
        ScuhbswenR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - SCU_HB_SW_CNT"]
    #[inline(always)]
    pub fn scuhbswcnt(&mut self) -> ScuhbswcntW<Scu0ccSpec> {
        ScuhbswcntW::new(self, 0)
    }
    #[doc = "Bit 16 - SCU_HB_SW_EN"]
    #[inline(always)]
    pub fn scuhbswen(&mut self) -> ScuhbswenW<Scu0ccSpec> {
        ScuhbswenW::new(self, 16)
    }
}
#[doc = "HeartBeat Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0ccSpec;
impl crate::RegisterSpec for Scu0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0cc::R`](R) reader structure"]
impl crate::Readable for Scu0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`scu0cc::W`](W) writer structure"]
impl crate::Writable for Scu0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0CC to value 0"]
impl crate::Resettable for Scu0ccSpec {}
