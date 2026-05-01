#[doc = "Register `HCIRHS034` reader"]
pub type R = crate::R<Hcirhs034Spec>;
#[doc = "Register `HCIRHS034` writer"]
pub type W = crate::W<Hcirhs034Spec>;
#[doc = "Field `REGCHUNKCOUNT` reader - REG_CHUNK_COUNT"]
pub type RegchunkcountR = crate::FieldReader<u16>;
#[doc = "Field `REGCHUNKCOUNT` writer - REG_CHUNK_COUNT"]
pub type RegchunkcountW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `REGCHUNKSIZE` reader - REG_CHUNK_SIZE"]
pub type RegchunksizeR = crate::FieldReader;
#[doc = "Field `REGCHUNKSIZE` writer - REG_CHUNK_SIZE"]
pub type RegchunksizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `REGIBISTATUSRINGSIZE` reader - REG_IBI_STATUS_RING_SIZE"]
pub type RegibistatusringsizeR = crate::FieldReader;
#[doc = "Field `REGIBISTATUSRINGSIZE` writer - REG_IBI_STATUS_RING_SIZE"]
pub type RegibistatusringsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGIBISTATUSSTRUCTSIZE` reader - REG_IBI_STATUS_STRUCT_SIZE"]
pub type RegibistatusstructsizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:9 - REG_CHUNK_COUNT"]
    #[inline(always)]
    pub fn regchunkcount(&self) -> RegchunkcountR {
        RegchunkcountR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:12 - REG_CHUNK_SIZE"]
    #[inline(always)]
    pub fn regchunksize(&self) -> RegchunksizeR {
        RegchunksizeR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 16:23 - REG_IBI_STATUS_RING_SIZE"]
    #[inline(always)]
    pub fn regibistatusringsize(&self) -> RegibistatusringsizeR {
        RegibistatusringsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_IBI_STATUS_STRUCT_SIZE"]
    #[inline(always)]
    pub fn regibistatusstructsize(&self) -> RegibistatusstructsizeR {
        RegibistatusstructsizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:9 - REG_CHUNK_COUNT"]
    #[inline(always)]
    pub fn regchunkcount(&mut self) -> RegchunkcountW<Hcirhs034Spec> {
        RegchunkcountW::new(self, 0)
    }
    #[doc = "Bits 10:12 - REG_CHUNK_SIZE"]
    #[inline(always)]
    pub fn regchunksize(&mut self) -> RegchunksizeW<Hcirhs034Spec> {
        RegchunksizeW::new(self, 10)
    }
    #[doc = "Bits 16:23 - REG_IBI_STATUS_RING_SIZE"]
    #[inline(always)]
    pub fn regibistatusringsize(&mut self) -> RegibistatusringsizeW<Hcirhs034Spec> {
        RegibistatusringsizeW::new(self, 16)
    }
}
#[doc = "IBI\\_SETUP\n\nYou can [`read`](crate::Reg::read) this register and get [`hcirhs034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcirhs034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcirhs034Spec;
impl crate::RegisterSpec for Hcirhs034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcirhs034::R`](R) reader structure"]
impl crate::Readable for Hcirhs034Spec {}
#[doc = "`write(|w| ..)` method takes [`hcirhs034::W`](W) writer structure"]
impl crate::Writable for Hcirhs034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIRHS034 to value 0x0400_0000"]
impl crate::Resettable for Hcirhs034Spec {
    const RESET_VALUE: u32 = 0x0400_0000;
}
