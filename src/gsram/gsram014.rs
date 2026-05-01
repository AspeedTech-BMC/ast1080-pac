#[doc = "Register `GSRAM014` reader"]
pub type R = crate::R<Gsram014Spec>;
#[doc = "Register `GSRAM014` writer"]
pub type W = crate::W<Gsram014Spec>;
#[doc = "Field `MAPADDR0` reader - MAP_ADDR0"]
pub type Mapaddr0R = crate::FieldReader<u16>;
#[doc = "Field `MAPADDR0` writer - MAP_ADDR0"]
pub type Mapaddr0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MAPADDR1` reader - MAP_ADDR1"]
pub type Mapaddr1R = crate::FieldReader<u16>;
#[doc = "Field `MAPADDR1` writer - MAP_ADDR1"]
pub type Mapaddr1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAP_ADDR0"]
    #[inline(always)]
    pub fn mapaddr0(&self) -> Mapaddr0R {
        Mapaddr0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - MAP_ADDR1"]
    #[inline(always)]
    pub fn mapaddr1(&self) -> Mapaddr1R {
        Mapaddr1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAP_ADDR0"]
    #[inline(always)]
    pub fn mapaddr0(&mut self) -> Mapaddr0W<Gsram014Spec> {
        Mapaddr0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - MAP_ADDR1"]
    #[inline(always)]
    pub fn mapaddr1(&mut self) -> Mapaddr1W<Gsram014Spec> {
        Mapaddr1W::new(self, 16)
    }
}
#[doc = "GSRAM\\_MAP\\_ADDR0\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram014Spec;
impl crate::RegisterSpec for Gsram014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram014::R`](R) reader structure"]
impl crate::Readable for Gsram014Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram014::W`](W) writer structure"]
impl crate::Writable for Gsram014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM014 to value 0x00ff_00fe"]
impl crate::Resettable for Gsram014Spec {
    const RESET_VALUE: u32 = 0x00ff_00fe;
}
