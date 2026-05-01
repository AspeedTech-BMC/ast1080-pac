#[doc = "Register `PRIC_IO214` reader"]
pub type R = crate::R<PricIo214Spec>;
#[doc = "Register `PRIC_IO214` writer"]
pub type W = crate::W<PricIo214Spec>;
#[doc = "Field `Reserved25` reader - Reserved"]
pub type Reserved25R = crate::BitReader;
#[doc = "Field `Reserved25` writer - Reserved"]
pub type Reserved25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved24` reader - Reserved"]
pub type Reserved24R = crate::BitReader;
#[doc = "Field `Reserved24` writer - Reserved"]
pub type Reserved24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved23` reader - Reserved"]
pub type Reserved23R = crate::BitReader;
#[doc = "Field `Reserved23` writer - Reserved"]
pub type Reserved23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved22` reader - Reserved"]
pub type Reserved22R = crate::BitReader;
#[doc = "Field `Reserved22` writer - Reserved"]
pub type Reserved22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved21` reader - Reserved"]
pub type Reserved21R = crate::BitReader;
#[doc = "Field `Reserved21` writer - Reserved"]
pub type Reserved21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved20` reader - Reserved"]
pub type Reserved20R = crate::BitReader;
#[doc = "Field `Reserved20` writer - Reserved"]
pub type Reserved20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved19` reader - Reserved"]
pub type Reserved19R = crate::BitReader;
#[doc = "Field `Reserved19` writer - Reserved"]
pub type Reserved19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved18` reader - Reserved"]
pub type Reserved18R = crate::BitReader;
#[doc = "Field `Reserved18` writer - Reserved"]
pub type Reserved18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved17` reader - Reserved"]
pub type Reserved17R = crate::BitReader;
#[doc = "Field `Reserved17` writer - Reserved"]
pub type Reserved17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved16` reader - Reserved"]
pub type Reserved16R = crate::BitReader;
#[doc = "Field `Reserved16` writer - Reserved"]
pub type Reserved16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved15` reader - Reserved"]
pub type Reserved15R = crate::BitReader;
#[doc = "Field `Reserved15` writer - Reserved"]
pub type Reserved15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved14` reader - Reserved"]
pub type Reserved14R = crate::BitReader;
#[doc = "Field `Reserved14` writer - Reserved"]
pub type Reserved14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved13` reader - Reserved"]
pub type Reserved13R = crate::BitReader;
#[doc = "Field `Reserved13` writer - Reserved"]
pub type Reserved13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved12` reader - Reserved"]
pub type Reserved12R = crate::BitReader;
#[doc = "Field `Reserved12` writer - Reserved"]
pub type Reserved12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::BitReader;
#[doc = "Field `Reserved11` writer - Reserved"]
pub type Reserved11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::BitReader;
#[doc = "Field `Reserved9` writer - Reserved"]
pub type Reserved9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `Reserved7` writer - Reserved"]
pub type Reserved7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved6` writer - Reserved"]
pub type Reserved6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved5` writer - Reserved"]
pub type Reserved5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved25(&self) -> Reserved25R {
        Reserved25R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved24(&self) -> Reserved24R {
        Reserved24R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved23(&self) -> Reserved23R {
        Reserved23R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved22(&self) -> Reserved22R {
        Reserved22R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved21(&self) -> Reserved21R {
        Reserved21R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved20(&self) -> Reserved20R {
        Reserved20R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved19(&self) -> Reserved19R {
        Reserved19R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved18(&self) -> Reserved18R {
        Reserved18R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved17(&self) -> Reserved17R {
        Reserved17R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved16(&self) -> Reserved16R {
        Reserved16R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&self) -> Reserved15R {
        Reserved15R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&self) -> Reserved14R {
        Reserved14R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&self) -> Reserved13R {
        Reserved13R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&self) -> Reserved12R {
        Reserved12R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn reserved25(&mut self) -> Reserved25W<PricIo214Spec> {
        Reserved25W::new(self, 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn reserved24(&mut self) -> Reserved24W<PricIo214Spec> {
        Reserved24W::new(self, 1)
    }
    #[doc = "Bit 2 - Reserved"]
    #[inline(always)]
    pub fn reserved23(&mut self) -> Reserved23W<PricIo214Spec> {
        Reserved23W::new(self, 2)
    }
    #[doc = "Bit 3 - Reserved"]
    #[inline(always)]
    pub fn reserved22(&mut self) -> Reserved22W<PricIo214Spec> {
        Reserved22W::new(self, 3)
    }
    #[doc = "Bit 4 - Reserved"]
    #[inline(always)]
    pub fn reserved21(&mut self) -> Reserved21W<PricIo214Spec> {
        Reserved21W::new(self, 4)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn reserved20(&mut self) -> Reserved20W<PricIo214Spec> {
        Reserved20W::new(self, 5)
    }
    #[doc = "Bit 6 - Reserved"]
    #[inline(always)]
    pub fn reserved19(&mut self) -> Reserved19W<PricIo214Spec> {
        Reserved19W::new(self, 6)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved18(&mut self) -> Reserved18W<PricIo214Spec> {
        Reserved18W::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved17(&mut self) -> Reserved17W<PricIo214Spec> {
        Reserved17W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn reserved16(&mut self) -> Reserved16W<PricIo214Spec> {
        Reserved16W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn reserved15(&mut self) -> Reserved15W<PricIo214Spec> {
        Reserved15W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn reserved14(&mut self) -> Reserved14W<PricIo214Spec> {
        Reserved14W::new(self, 11)
    }
    #[doc = "Bit 12 - Reserved"]
    #[inline(always)]
    pub fn reserved13(&mut self) -> Reserved13W<PricIo214Spec> {
        Reserved13W::new(self, 12)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn reserved12(&mut self) -> Reserved12W<PricIo214Spec> {
        Reserved12W::new(self, 13)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&mut self) -> Reserved11W<PricIo214Spec> {
        Reserved11W::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<PricIo214Spec> {
        Reserved10W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&mut self) -> Reserved9W<PricIo214Spec> {
        Reserved9W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<PricIo214Spec> {
        Reserved8W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&mut self) -> Reserved7W<PricIo214Spec> {
        Reserved7W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&mut self) -> Reserved6W<PricIo214Spec> {
        Reserved6W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&mut self) -> Reserved5W<PricIo214Spec> {
        Reserved5W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<PricIo214Spec> {
        Reserved4W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<PricIo214Spec> {
        Reserved3W::new(self, 22)
    }
    #[doc = "Bit 23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<PricIo214Spec> {
        Reserved2W::new(self, 23)
    }
    #[doc = "Bits 24:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<PricIo214Spec> {
        Reserved1W::new(self, 24)
    }
}
#[doc = "Slave Write Group Setting Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`pric_io214::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pric_io214::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PricIo214Spec;
impl crate::RegisterSpec for PricIo214Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pric_io214::R`](R) reader structure"]
impl crate::Readable for PricIo214Spec {}
#[doc = "`write(|w| ..)` method takes [`pric_io214::W`](W) writer structure"]
impl crate::Writable for PricIo214Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIC_IO214 to value 0x3f3f_3f3f"]
impl crate::Resettable for PricIo214Spec {
    const RESET_VALUE: u32 = 0x3f3f_3f3f;
}
