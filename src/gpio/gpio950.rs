#[doc = "Register `GPIO950` reader"]
pub type R = crate::R<Gpio950Spec>;
#[doc = "Register `GPIO950` writer"]
pub type W = crate::W<Gpio950Spec>;
#[doc = "Field `GPIO064ReadPrivilegeOfMaster` reader - GPIO064 Read Privilege of Master"]
pub type Gpio064readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO064ReadPrivilegeOfMaster` writer - GPIO064 Read Privilege of Master"]
pub type Gpio064readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO065ReadPrivilegeOfMaster` reader - GPIO065 Read Privilege of Master"]
pub type Gpio065readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO065ReadPrivilegeOfMaster` writer - GPIO065 Read Privilege of Master"]
pub type Gpio065readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO066ReadPrivilegeOfMaster` reader - GPIO066 Read Privilege of Master"]
pub type Gpio066readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO066ReadPrivilegeOfMaster` writer - GPIO066 Read Privilege of Master"]
pub type Gpio066readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO067ReadPrivilegeOfMaster` reader - GPIO067 Read Privilege of Master"]
pub type Gpio067readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO067ReadPrivilegeOfMaster` writer - GPIO067 Read Privilege of Master"]
pub type Gpio067readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO064 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio064read_privilege_of_master(&self) -> Gpio064readPrivilegeOfMasterR {
        Gpio064readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO065 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio065read_privilege_of_master(&self) -> Gpio065readPrivilegeOfMasterR {
        Gpio065readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO066 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio066read_privilege_of_master(&self) -> Gpio066readPrivilegeOfMasterR {
        Gpio066readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO067 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio067read_privilege_of_master(&self) -> Gpio067readPrivilegeOfMasterR {
        Gpio067readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO064 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio064read_privilege_of_master(
        &mut self,
    ) -> Gpio064readPrivilegeOfMasterW<Gpio950Spec> {
        Gpio064readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO065 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio065read_privilege_of_master(
        &mut self,
    ) -> Gpio065readPrivilegeOfMasterW<Gpio950Spec> {
        Gpio065readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO066 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio066read_privilege_of_master(
        &mut self,
    ) -> Gpio066readPrivilegeOfMasterW<Gpio950Spec> {
        Gpio066readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO067 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio067read_privilege_of_master(
        &mut self,
    ) -> Gpio067readPrivilegeOfMasterW<Gpio950Spec> {
        Gpio067readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio950::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio950::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio950Spec;
impl crate::RegisterSpec for Gpio950Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio950::R`](R) reader structure"]
impl crate::Readable for Gpio950Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio950::W`](W) writer structure"]
impl crate::Writable for Gpio950Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO950 to value 0xffff_ffff"]
impl crate::Resettable for Gpio950Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
