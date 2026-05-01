#[doc = "Register `GPIO980` reader"]
pub type R = crate::R<Gpio980Spec>;
#[doc = "Register `GPIO980` writer"]
pub type W = crate::W<Gpio980Spec>;
#[doc = "Field `GPIO112ReadPrivilegeOfMaster` reader - GPIO112 Read Privilege of Master"]
pub type Gpio112readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO112ReadPrivilegeOfMaster` writer - GPIO112 Read Privilege of Master"]
pub type Gpio112readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO113ReadPrivilegeOfMaster` reader - GPIO113 Read Privilege of Master"]
pub type Gpio113readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO113ReadPrivilegeOfMaster` writer - GPIO113 Read Privilege of Master"]
pub type Gpio113readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO114ReadPrivilegeOfMaster` reader - GPIO114 Read Privilege of Master"]
pub type Gpio114readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO114ReadPrivilegeOfMaster` writer - GPIO114 Read Privilege of Master"]
pub type Gpio114readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO115ReadPrivilegeOfMaster` reader - GPIO115 Read Privilege of Master"]
pub type Gpio115readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO115ReadPrivilegeOfMaster` writer - GPIO115 Read Privilege of Master"]
pub type Gpio115readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO112 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio112read_privilege_of_master(&self) -> Gpio112readPrivilegeOfMasterR {
        Gpio112readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO113 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio113read_privilege_of_master(&self) -> Gpio113readPrivilegeOfMasterR {
        Gpio113readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO114 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio114read_privilege_of_master(&self) -> Gpio114readPrivilegeOfMasterR {
        Gpio114readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO115 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio115read_privilege_of_master(&self) -> Gpio115readPrivilegeOfMasterR {
        Gpio115readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO112 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio112read_privilege_of_master(
        &mut self,
    ) -> Gpio112readPrivilegeOfMasterW<Gpio980Spec> {
        Gpio112readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO113 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio113read_privilege_of_master(
        &mut self,
    ) -> Gpio113readPrivilegeOfMasterW<Gpio980Spec> {
        Gpio113readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO114 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio114read_privilege_of_master(
        &mut self,
    ) -> Gpio114readPrivilegeOfMasterW<Gpio980Spec> {
        Gpio114readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO115 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio115read_privilege_of_master(
        &mut self,
    ) -> Gpio115readPrivilegeOfMasterW<Gpio980Spec> {
        Gpio115readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#28\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio980::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio980::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio980Spec;
impl crate::RegisterSpec for Gpio980Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio980::R`](R) reader structure"]
impl crate::Readable for Gpio980Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio980::W`](W) writer structure"]
impl crate::Writable for Gpio980Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO980 to value 0xffff_ffff"]
impl crate::Resettable for Gpio980Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
