#[doc = "Register `GSRAM000` reader"]
pub type R = crate::R<Gsram000Spec>;
#[doc = "Register `GSRAM000` writer"]
pub type W = crate::W<Gsram000Spec>;
#[doc = "Field `SSRAMEN` reader - SSRAM_EN"]
pub type SsramenR = crate::FieldReader;
#[doc = "Field `SSRAMEN` writer - SSRAM_EN"]
pub type SsramenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `OSRAMEN` reader - OSRAM_EN"]
pub type OsramenR = crate::FieldReader;
#[doc = "Field `OSRAMEN` writer - OSRAM_EN"]
pub type OsramenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SSRAM_EN"]
    #[inline(always)]
    pub fn ssramen(&self) -> SsramenR {
        SsramenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - OSRAM_EN"]
    #[inline(always)]
    pub fn osramen(&self) -> OsramenR {
        OsramenR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SSRAM_EN"]
    #[inline(always)]
    pub fn ssramen(&mut self) -> SsramenW<Gsram000Spec> {
        SsramenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - OSRAM_EN"]
    #[inline(always)]
    pub fn osramen(&mut self) -> OsramenW<Gsram000Spec> {
        OsramenW::new(self, 8)
    }
}
#[doc = "GSRAM\\_CTL\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram000Spec;
impl crate::RegisterSpec for Gsram000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram000::R`](R) reader structure"]
impl crate::Readable for Gsram000Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram000::W`](W) writer structure"]
impl crate::Writable for Gsram000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM000 to value 0xff"]
impl crate::Resettable for Gsram000Spec {
    const RESET_VALUE: u32 = 0xff;
}
