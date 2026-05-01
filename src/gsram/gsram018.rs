#[doc = "Register `GSRAM018` reader"]
pub type R = crate::R<Gsram018Spec>;
#[doc = "Register `GSRAM018` writer"]
pub type W = crate::W<Gsram018Spec>;
#[doc = "Field `MAPADDR2` reader - MAP_ADDR2"]
pub type Mapaddr2R = crate::FieldReader<u16>;
#[doc = "Field `MAPADDR2` writer - MAP_ADDR2"]
pub type Mapaddr2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAPADDR3` reader - MAP_ADDR3"]
pub type Mapaddr3R = crate::FieldReader<u16>;
#[doc = "Field `MAPADDR3` writer - MAP_ADDR3"]
pub type Mapaddr3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAP_ADDR2"]
    #[inline(always)]
    pub fn mapaddr2(&self) -> Mapaddr2R {
        Mapaddr2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MAP_ADDR3"]
    #[inline(always)]
    pub fn mapaddr3(&self) -> Mapaddr3R {
        Mapaddr3R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAP_ADDR2"]
    #[inline(always)]
    pub fn mapaddr2(&mut self) -> Mapaddr2W<Gsram018Spec> {
        Mapaddr2W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MAP_ADDR3"]
    #[inline(always)]
    pub fn mapaddr3(&mut self) -> Mapaddr3W<Gsram018Spec> {
        Mapaddr3W::new(self, 16)
    }
}
#[doc = "GSRAM\\_MAP\\_ADDR1\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram018Spec;
impl crate::RegisterSpec for Gsram018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram018::R`](R) reader structure"]
impl crate::Readable for Gsram018Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram018::W`](W) writer structure"]
impl crate::Writable for Gsram018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM018 to value 0x0100"]
impl crate::Resettable for Gsram018Spec {
    const RESET_VALUE: u32 = 0x0100;
}
