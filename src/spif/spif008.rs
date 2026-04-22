#[doc = "Register `SPIF008` reader"]
pub type R = crate::R<Spif008Spec>;
#[doc = "Register `SPIF008` writer"]
pub type W = crate::W<Spif008Spec>;
#[doc = "Field `EAR` reader - EAR"]
pub type EarR = crate::FieldReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `OSTHRES` reader - OS_THRES"]
pub type OsthresR = crate::FieldReader;
#[doc = "Field `OSTHRES` writer - OS_THRES"]
pub type OsthresW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OSEN` reader - OS_EN"]
pub type OsenR = crate::BitReader;
#[doc = "Field `OSEN` writer - OS_EN"]
pub type OsenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - EAR"]
    #[inline(always)]
    pub fn ear(&self) -> EarR {
        EarR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - OS_THRES"]
    #[inline(always)]
    pub fn osthres(&self) -> OsthresR {
        OsthresR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 31 - OS_EN"]
    #[inline(always)]
    pub fn osen(&self) -> OsenR {
        OsenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - OS_THRES"]
    #[inline(always)]
    pub fn osthres(&mut self) -> OsthresW<Spif008Spec> {
        OsthresW::new(self, 16)
    }
    #[doc = "Bit 31 - OS_EN"]
    #[inline(always)]
    pub fn osen(&mut self) -> OsenW<Spif008Spec> {
        OsenW::new(self, 31)
    }
}
#[doc = "SPIF\\_OS\n\nYou can [`read`](crate::Reg::read) this register and get [`spif008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif008Spec;
impl crate::RegisterSpec for Spif008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif008::R`](R) reader structure"]
impl crate::Readable for Spif008Spec {}
#[doc = "`write(|w| ..)` method takes [`spif008::W`](W) writer structure"]
impl crate::Writable for Spif008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF008 to value 0"]
impl crate::Resettable for Spif008Spec {}
